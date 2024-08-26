#include <fcntl.h>
#include <liburing.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>

#define QUEUE_DEPTH 1
#define BLOCK_SZ 1024

struct file_info {
  off_t blocks;
  struct iovec iovecs[];
};

struct file_info *create_file_info(off_t size) {
  off_t blocks = (size % BLOCK_SZ ? 1 : 0) + (size / BLOCK_SZ);
  struct file_info *fi =
      malloc(sizeof(struct file_info) + (sizeof(struct iovec) * blocks));
  fi->blocks = blocks;

  return fi;
}

off_t get_file_size(int fd) {
  struct stat st;
  if (fstat(fd, &st) < 0) {
    perror("fstat");
    return -1;
  }

  return st.st_size;
}

int submit_read_request(const int fd, struct io_uring *ring) {
  off_t file_size = get_file_size(fd);
  if (file_size == -1) {
    return EXIT_FAILURE;
  }

#pragma region Allocate iovec
  struct file_info *fi = create_file_info(file_size);

  off_t bytes_remaining = file_size;
  int current_block = 0;
  while (bytes_remaining > 0) {
    off_t bytes_to_read = bytes_remaining;
    if (bytes_to_read > BLOCK_SZ) {
      bytes_to_read = BLOCK_SZ;
    }

    void *buf;
    if (posix_memalign(&buf, BLOCK_SZ, BLOCK_SZ)) {
      perror("posix_memalign");
      return EXIT_FAILURE;
    }
    fi->iovecs[current_block].iov_len = bytes_to_read;
    fi->iovecs[current_block].iov_base = buf;

    current_block++;
    bytes_remaining -= bytes_to_read;
  }
#pragma endregion

#pragma region Submit io_uring request
  struct io_uring_sqe *sqe = io_uring_get_sqe(ring);
  io_uring_prep_readv(sqe, fd, fi->iovecs, fi->blocks, 0);
  io_uring_sqe_set_data(sqe, fi);
  io_uring_submit(ring);
#pragma endregion

  return EXIT_SUCCESS;
}

int pull_completed_request(struct io_uring *ring) {
  struct io_uring_cqe *cqe;
  int ret = io_uring_wait_cqe(ring, &cqe);
  if (ret) {
    perror("io_uring_wait_cqe");
    return EXIT_FAILURE;
  }

  if (cqe->res < 0) {
    fprintf(stderr, "readv failed.\n");
    return EXIT_FAILURE;
  }

  struct file_info *fi = io_uring_cqe_get_data(cqe);
  for (int i = 0; i < fi->blocks; i++) {
    fwrite(fi->iovecs[i].iov_base, fi->iovecs->iov_len, 1, stdout);
  }

  io_uring_cqe_seen(ring, cqe);

  return EXIT_SUCCESS;
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    fprintf(stderr, "Usage: %s [filename] <...[filename]>\n", argv[0]);
    return EXIT_FAILURE;
  }

  struct io_uring ring;
  io_uring_queue_init(QUEUE_DEPTH, &ring, 0);

  for (int i = 1; i < argc; i++) {
    int fd = open(argv[i], O_RDONLY);
    if (fd < 0) {
      perror("open");
      return EXIT_FAILURE;
    }
    if (submit_read_request(fd, &ring)) {
      fprintf(stderr, "Failed to read file %s\n", argv[i]);
      return EXIT_FAILURE;
    }
    pull_completed_request(&ring);
    close(fd);
  }

  io_uring_queue_exit(&ring);
  return EXIT_SUCCESS;
}

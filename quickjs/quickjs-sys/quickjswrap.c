#include "quickjswrap.h"

void JS_FreeValue__extern(JSContext *ctx, JSValue v)
{
  JS_FreeValue(ctx, v);
}

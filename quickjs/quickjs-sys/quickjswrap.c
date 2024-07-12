#include "quickjswrap.h"

void JS_FreeValue__extern(JSContext *ctx, JSValue v)
{
  JS_FreeValue(ctx, v);
}

const char *JS_ToCString__extern(JSContext *ctx, JSValueConst val1)
{
  return JS_ToCString(ctx, val1);
}
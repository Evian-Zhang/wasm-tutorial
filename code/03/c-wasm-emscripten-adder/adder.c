#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE int add(int left, int right) {
    return left + right;
}

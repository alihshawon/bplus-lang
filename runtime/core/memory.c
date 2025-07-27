// runtime/core/memory.c
#include <stdlib.h>

void* bplus_alloc(size_t size) {
    // In a real runtime, this would be tied to the GC
    return malloc(size);
}

void bplus_free(void* ptr) {
    free(ptr);
}
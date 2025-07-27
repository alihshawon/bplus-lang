// runtime/core/os.c
#include <stdio.h>
#include <time.h>

long bplus_os_time() {
    return time(NULL);
}
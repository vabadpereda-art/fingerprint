#pragma once

#ifdef _WIN32

#ifndef MAXPATHLEN
#define MAXPATHLEN 260
#endif

#ifndef MAX
#define MAX(a, b) (((a) > (b)) ? (a) : (b))
#endif

#ifndef MIN
#define MIN(a, b) (((a) < (b)) ? (a) : (b))
#endif

#else
#include_next <sys/param.h>
#endif

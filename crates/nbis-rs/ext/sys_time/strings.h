#pragma once

#ifdef _WIN32
#include <string.h>

#ifndef strcasecmp
#define strcasecmp _stricmp
#endif

#ifndef strncasecmp
#define strncasecmp _strnicmp
#endif

#ifndef bzero
#define bzero(ptr, size) memset((ptr), 0, (size))
#endif

#ifndef bcopy
#define bcopy(src, dst, size) memmove((dst), (src), (size))
#endif

#ifndef bcmp
#define bcmp(ptr1, ptr2, size) memcmp((ptr1), (ptr2), (size))
#endif

#else
#include_next <strings.h>
#endif

#pragma once

#ifdef _WIN32

#include <direct.h>
#include <io.h>
#include <process.h>
#include <stdlib.h>

#ifndef _PID_T_
#ifndef _PID_T_DEFINED
typedef int pid_t;
#define _PID_T_
#define _PID_T_DEFINED
#endif
#endif

#ifndef F_OK
#define F_OK 0
#endif
#ifndef X_OK
#define X_OK 1
#endif
#ifndef W_OK
#define W_OK 2
#endif
#ifndef R_OK
#define R_OK 4
#endif

#ifndef STDIN_FILENO
#define STDIN_FILENO 0
#endif
#ifndef STDOUT_FILENO
#define STDOUT_FILENO 1
#endif
#ifndef STDERR_FILENO
#define STDERR_FILENO 2
#endif

#define access _access
#define chdir _chdir
#define close _close
#define getcwd _getcwd
#define getpid _getpid
#define read _read
#define rmdir _rmdir
#define unlink _unlink
#define write _write
#define sleep(seconds) (_sleep((seconds) * 1000), 0)

#endif /* _WIN32 */

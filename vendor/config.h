/* config.h for libcdio */

#pragma once

#define LIBCDIO_CONFIG_H

/* Standard headers available on all modern platforms */
#define HAVE_ERRNO_H
#define HAVE_FCNTL_H
#define HAVE_INTTYPES_H
#define HAVE_LIMITS_H
#define HAVE_MEMORY_H
#define HAVE_MEMCPY
#define HAVE_MEMSET
#define HAVE_SNPRINTF
#define HAVE_STDBOOL_H
#define HAVE_STDARG_H
#define HAVE_STDINT_H
#define HAVE_STDIO_H
#define HAVE_STDLIB_H
#define HAVE_STRING_H
#define HAVE_VSNPRINTF
#define HAVE_SYS_STAT_H
#define HAVE_SYS_TYPES_H
#define STDC_HEADERS

/* Common values */
#define EMPTY_ARRAY_SIZE
#define ICONV_CONST
#define HAVE_JOLIET
#define HAVE_ROCK
#define HAVE_UNISTD_H /* A custom unistd.h is used for MSVC */

/* Platform specific */
#if defined(_WIN32)
    #define HAVE_NTDDSCSI_H
    #define HAVE_NTDDCDRM_H
    #define HAVE_WIN32_CDROM
    #define HAVE_WINDOWS_H
#endif
#if defined(__APPLE__) || (__FreeBSD__) || (__linux__)
    #define HAVE_DLFCN_H
    #define HAVE_GLOB_H
    #define HAVE_GMTIME_R
    #define HAVE_ICONV
    #define HAVE_LOCALTIME_R
    #define HAVE_SETENV
    #define HAVE_STRINGS_H
    #define HAVE_STRTOK_R
    #define HAVE_TM_GMTOFF
    #define HAVE_TIMEGM
    #define HAVE_UNSETENV
#endif
#if defined(__APPLE__)
    #define HAVE_COREFOUNDATION_CFBASE_H
    #define HAVE_DARWIN_CDROM
    #define HAVE_DISKARBITRATION
    #define HAVE_IOKIT_IOKITLIB_H
#endif
#if defined(__FreeBSD__)
    #define HAVE_FREEBSD_CDROM
    #define HAVE_SYS_CDIO_H
#endif
#if defined (__linux__)
    #define _FILE_OFFSET_BITS 64
    #define HAVE_LINUX_CDROM
    #define HAVE_LINUX_VERSION_H
#endif

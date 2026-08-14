/*
 * `version.h` to be used in place of autoconf-generated `version.h.in`
 * for libcdio's Cargo builds
 */

#define CDIO_VERSION "2.4.0 cargo-build"
extern const char *cdio_version_string;

#define LIBCDIO_VERSION_NUM 20400
extern const unsigned int libcdio_version_num;

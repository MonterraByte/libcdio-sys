# libcdio-sys
Native bindings to the [libcdio](https://github.com/libcdio/libcdio) and
[libcdio-paranoia](https://github.com/libcdio/libcdio-paranoia) libraries.

[![crates.io](https://img.shields.io/crates/v/libcdio-sys.svg)](https://crates.io/crates/libcdio-sys)
[![Documentation](https://docs.rs/libcdio-sys/badge.svg)](https://docs.rs/libcdio-sys)

# Usage
Run `cargo add libcdio-sys` to add the crate.

libcdio is split into multiple libraries: `libcdio`, `libiso9660` and
`libudf`. Likewise, libcdio-paranoia is split into `libcdio_cdda` and
`libcdio_paranoia`.

Cargo features are provided to control which libraries are included.

## Versioning
The crate's version reflects three things:
For example: `v3.0.0+2.4.0p2.0.2`.
- `v3.0.0`: Crate version
- `+2.4.0`: libcdio's version
- `p2.0.2`: libcdio-paranoia's version (`10.2+2.0.2` in this case)
Everything beyond the `+` is ignored by Cargo during resolution.

## Vendoring
Enable the `vendored` feature to always perform vendored builds.

If disabled, the crate will attempt to use the libraries from the
system before falling back to vendoring if that fails.
To force a non-vendored build, set the `LIBCDIO_NO_VENDOR` environment
variable.

Non-vendored builds use the version metadata as a lower bound
when probing the system, allowing any newer non-major version.
Thus, a non-vendored build for `v3.0.0+2.4.0p2.0.2` would probe the
system for libcdio `>= 2.4.0, < 3.0.0` and (if features are enabled)
libcdio-paranoia `>= 10.2+2.0.2, < 10.2+3.0.0`.

You need to have libcdio and its headers installed for non-vendored builds.

pkg-config is also required for this, though this requirement can be
avoided by setting the following environment variables:

| Feature    | Environment variables                                                                         |
| ---------- | --------------------------------------------------------------------------------------------- |
| none       | `SYSTEM_DEPS_LIBCDIO_NO_PKG_CONFIG=1 SYSTEM_DEPS_LIBCDIO_LIB=cdio`                            |
| `iso9660`  | `SYSTEM_DEPS_LIBISO9660_NO_PKG_CONFIG=1 SYSTEM_DEPS_LIBISO9660_LIB=iso9660`                   |
| `udf`      | `SYSTEM_DEPS_LIBUDF_NO_PKG_CONFIG=1 SYSTEM_DEPS_LIBUDF_LIB=udf`                               |
| `cdda`     | `SYSTEM_DEPS_LIBCDIO_CDDA_NO_PKG_CONFIG=1 SYSTEM_DEPS_LIBCDIO_CDDA_LIB=cdio_cdda`             |
| `paranoia` | `SYSTEM_DEPS_LIBCDIO_PARANOIA_NO_PKG_CONFIG=1 SYSTEM_DEPS_LIBCDIO_PARANOIA_LIB=cdio_paranoia` |

### Overriding the system library
To control the shared library used in a non-vendored build, set the
`PKG_CONFIG_PATH` environment variable to the path of the directory
containing pkg-config files for the correct library.
Alternatively, set the the following environment variables, depending
on which features you have enabled:

| Feature    | Library path                                 | Include path                           |
| ---------- | -------------------------------------------- | -------------------------------------- |
| none       | `SYSTEM_DEPS_LIBCDIO_SEARCH_NATIVE`          | `SYSTEM_DEPS_LIBCDIO_INCLUDE`          |
| `iso9660`  | `SYSTEM_DEPS_LIBISO9660_SEARCH_NATIVE`       | `SYSTEM_DEPS_LIBISO9660_INCLUDE`       |
| `udf`      | `SYSTEM_DEPS_LIBUDF_SEARCH_NATIVE`           | `SYSTEM_DEPS_LIBUDF_INCLUDE`           |
| `cdda`     | `SYSTEM_DEPS_LIBCDIO_CDDA_SEARCH_NATIVE`     | `SYSTEM_DEPS_LIBCDIO_CDDA_INCLUDE`     |
| `paranoia` | `SYSTEM_DEPS_LIBCDIO_PARANOIA_SEARCH_NATIVE` | `SYSTEM_DEPS_LIBCDIO_PARANOIA_INCLUDE` |

Variables in the "Library path" column should be set to the path of the directory containing the library, and variables in the "Include path" column should be set to the path of the directory containing the `cdio` header directory.
See the [system-deps documentation](https://docs.rs/system-deps/7/system_deps/) for more information.

# License
Copyright © 2018, 2019, 2020, 2023, 2025 Joaquim Monteiro

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

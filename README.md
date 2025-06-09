# libcdio-sys

Native bindings to the libcdio and libcdio-paranoia libraries

[![crates.io](https://img.shields.io/crates/v/libcdio-sys.svg)](https://crates.io/crates/libcdio-sys)
[![Documentation](https://docs.rs/libcdio-sys/badge.svg)](https://docs.rs/libcdio-sys)

[libcdio homepage](https://www.gnu.org/software/libcdio/)  
[libcdio documentation](https://www.gnu.org/software/libcdio/libcdio.html)

# Prerequisites

You need to have libcdio and its headers installed in order to build this crate.

# Usage

Run `cargo add libcdio-sys`, or add `libcdio-sys = "0.5"` to the `[dependencies]` section of your Cargo.toml.

libcdio is split into multiple libraries: `libcdio`, `libiso9660` and `libudf`. (Likewise, libcdio-paranoia is split into `libcdio_cdda` and `libcdio_paranoia`.) The `libcdio-sys` crate always links against `libcdio`, and can link against the others depending on which Cargo features are enabled.

The available Cargo features are `iso9660`, `udf`, `cdda` and `paranoia`. The first two are enabled by default; the last two require libcdio-paranoia, which is usually installed separately from libcdio.

# License

Copyright © 2018, 2019, 2020, 2023 Joaquim Monteiro

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

# libcdio-sys

Native bindings to the libcdio library

[![Documentation](https://docs.rs/libcdio-sys/badge.svg)](https://docs.rs/libcdio-sys)

[libcdio homepage](https://www.gnu.org/software/libcdio/)  
[libcdio documentation](https://www.gnu.org/software/libcdio/libcdio.html)

# Prerequisites

You need to have libcdio and its headers installed in order to build this crate.

# Usage

Add the following to your `Cargo.toml`:

    [dependencies]
    libcdio-sys = "0.5"

Then, add the following to your crate root:

    extern crate libcdio_sys;

By default, `libcdio-sys` will link to libcdio, libiso9660 and libudf. You can control which libraries are linked using the following:

    [dependencies]
    libcdio-sys = { version = "0.5", default-features = false, features = ["cdio"] }

In this example, only libcdio is used. Valid features are: `cdio`, `iso9660` and `udf`.

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

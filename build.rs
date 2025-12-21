use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

// libcdio uses a homegrown boolean type for versions < 2.1.1.
// The homegrown boolean type is not recognized by bindgen.
// This would result in different code gen for versions < 2.1.1 and versions >= 2.1.1.
// To prevent this, we include stdbool.h ourselves, which suppresses the homegrown boolean type.
const CDIO_HEADER: &str = "#include <stdbool.h>
#include <cdio/cdio.h>
#include <cdio/cd_types.h>
#include <cdio/logging.h>
#include <cdio/mmc_cmds.h>
#include <cdio/utf8.h>\n";

#[cfg(feature = "iso9660")]
const ISO9660_HEADER: &str = "#include <cdio/iso9660.h>\n";

#[cfg(feature = "udf")]
const UDF_HEADER: &str = "#include <cdio/udf.h>\n";

#[cfg(feature = "cdda")]
const CDDA_HEADER: &str = "#include <cdio/paranoia/cdda.h>\n";

#[cfg(feature = "paranoia")]
const PARANOIA_HEADER: &str = "#include <cdio/paranoia/paranoia.h>\n";

const HEADERS: &[&str] = &[
    CDIO_HEADER,
    #[cfg(feature = "iso9660")]
    ISO9660_HEADER,
    #[cfg(feature = "udf")]
    UDF_HEADER,
    #[cfg(feature = "cdda")]
    CDDA_HEADER,
    #[cfg(feature = "paranoia")]
    PARANOIA_HEADER,
];

const LINK_LIBS: &[&str] = &[
    "cdio",
    #[cfg(feature = "iso9660")]
    "iso9660",
    #[cfg(feature = "udf")]
    "udf",
    #[cfg(feature = "cdda")]
    "cdio_cdda",
    #[cfg(feature = "paranoia")]
    "cdio_paranoia",
];

fn main() {
    let mut stdout = io::stdout().lock();
    for lib in LINK_LIBS {
        for s in [b"cargo:rustc-link-lib=", lib.as_bytes(), b"\n"] {
            stdout.write_all(s).unwrap();
        }
    }
    drop(stdout);

    let headers = HEADERS.join("");
    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", &headers)
        .allowlist_file(r".*[/\\]cdio[/\\][^/\\]*\.h")
        .allowlist_file(r".*[/\\]cdio[/\\]paranoia[/\\][^/\\]*\.h")
        .wrap_unsafe_ops(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

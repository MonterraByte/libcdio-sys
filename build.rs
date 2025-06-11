use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

const CDIO_HEADER: &str = "#include <cdio/cdio.h>
#include <cdio/cd_types.h>
#include <cdio/logging.h>
#include <cdio/mmc_cmds.h>
#include <cdio/utf8.h>\n";

const ISO9660_HEADER: &str = "#include <cdio/iso9660.h>\n";

const UDF_HEADER: &str = "#include <cdio/udf.h>\n";

const LINK_LIBS: &[&str] = &[
    "cdio",
    #[cfg(feature = "iso9660")]
    "iso9660",
    #[cfg(feature = "udf")]
    "udf",
];

fn main() {
    let mut header = String::from(CDIO_HEADER);

    #[cfg(feature = "iso9660")]
    header.push_str(ISO9660_HEADER);

    #[cfg(feature = "udf")]
    header.push_str(UDF_HEADER);

    let mut stdout = io::stdout().lock();
    for lib in LINK_LIBS {
        for s in [b"cargo:rustc-link-lib=", lib.as_bytes(), b"\n"] {
            stdout.write_all(s).unwrap();
        }
    }
    drop(stdout);

    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", header.as_str())
        .allowlist_file(r".*[/\\]cdio[/\\][^/\\]*\.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

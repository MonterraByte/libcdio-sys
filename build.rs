use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    system_deps::Config::new().probe()?;
    make_bindings(std::iter::empty::<&Path>())?;

    Ok(())
}

/// Create bindings at OUT_DIR/bindings.rs.
/// Additional include paths can be passed as input.
fn make_bindings(
    includes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> Result<(), Box<dyn Error>> {
    let mut builder = bindgen::Builder::default();
    for include in includes {
        builder = builder.clang_arg(format!("-I{}", include.as_ref().display()));
    }

    // libcdio uses a homegrown boolean type for versions < 2.1.1.
    // The homegrown boolean type is not recognized by bindgen.
    // This would result in different code gen for versions < 2.1.1 and versions >= 2.1.1.
    // To prevent this, we include stdbool.h ourselves, which suppresses the homegrown boolean type.
    static CDIO_HEADERS: &str = r"
        #include <stdbool.h>
        #include <cdio/cdio.h>
        #include <cdio/cd_types.h>
        #include <cdio/logging.h>
        #include <cdio/mmc_cmds.h>
        #include <cdio/utf8.h>
";
    static HEADERS: &[&str] = &[
        CDIO_HEADERS,
        #[cfg(feature = "iso9660")]
        "#include <cdio/iso9660.h>",
        #[cfg(feature = "udf")]
        "#include <cdio/udf.h>",
        #[cfg(feature = "cdda")]
        "#include <cdio/paranoia/cdda.h>",
        #[cfg(feature = "paranoia")]
        "#include <cdio/paranoia/paranoia.h>",
    ];
    let headers = HEADERS.join("\n");
    let bindings = builder
        .header_contents("wrapper.h", &headers)
        .allowlist_file(r".*[/\\]cdio[/\\][^/\\]*\.h")
        .allowlist_file(r".*[/\\]cdio[/\\]paranoia[/\\][^/\\]*\.h")
        .wrap_unsafe_ops(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should have been set by Cargo"));
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

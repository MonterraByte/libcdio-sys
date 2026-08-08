use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let vendoring_allowed = matches!(
        env::var("LIBCDIO_NO_VENDOR").as_deref(),
        Ok("") | Ok("0") | Err(_)
    );
    println!("cargo::rerun-if-env-changed=LIBCDIO_NO_VENDOR");

    if cfg!(feature = "vendored") && vendoring_allowed {
        make_bindings(make_static()?)
    } else {
        match system_deps::Config::new().probe() {
            Ok(deps) => make_bindings(deps.all_include_paths()),
            Err(err) if vendoring_allowed => {
                println!("cargo::warning=could not find libcdio");
                for line in err.to_string().lines() {
                    println!("cargo::warning={}", line);
                }
                println!(
                    "cargo::warning=LIBCDIO_NO_VENDOR is not set, falling back to a vendored build"
                );
                make_bindings(make_static()?)
            }
            Err(err) => {
                println!("cargo::warning=LIBCDIO_NO_VENDOR is set but libcdio could not be found");
                Err(err.into())
            }
        }
    }
}

/// Build a static library and return the (public) include paths
fn make_static() -> Result<Vec<PathBuf>, cc::Error> {
    let mut includes = Vec::new();
    let mut build = cc::Build::new();
    build.warnings(false);
    build.define("HAVE_CONFIG_H", None); // provided at vendor/config.h

    let vendor = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendor/");
    println!("cargo::rerun-if-changed=vendor/");

    // a location in OUT_DIR to place custom headers
    let include = env::var("OUT_DIR")
        .map(|out_dir| PathBuf::from(out_dir).join("include/"))
        .expect("OUT_DIR should have been set by cargo");
    includes.push(include.clone());
    cp(vendor.join("config.h"), include.join("config.h"))?;
    cp(vendor.join("version.h"), include.join("cdio/version.h"))?;
    let target = env::var("TARGET").expect("TARGET should have been set by Cargo");
    if target.contains("msvc") {
        // libcdio uses a custom unistd.h for its MSVC builds
        cp(
            vendor.join("libcdio/.vs/unistd.h"),
            include.join("unistd.h"),
        )?;
    }

    if target.contains("windows") {
        println!("cargo::rustc-link-lib=winmm");
    }
    if target.contains("darwin") {
        println!("cargo::rustc-link-lib=framework=CoreFoundation");
        println!("cargo::rustc-link-lib=framework=DiskArbitration");
        println!("cargo::rustc-link-lib=framework=IOKit");
    }
    if target.contains("freebsd") {
        println!("cargo::rustc-link-lib=cam");
    }
    includes.push(vendor.join("libcdio/include/"));
    let cdio_src = vendor.join("libcdio/lib/driver/");
    build.include(&cdio_src); // lets the compiler discover private headers
    build.files(CDIO_SOURCES.iter().map(|s| cdio_src.join(s)));
    if cfg!(feature = "iso9660") {
        let src = vendor.join("libcdio/lib/iso9660/");
        build.include(&src);
        build.files(ISO9660_SOURCES.iter().map(|s| src.join(s)));
    }
    if cfg!(feature = "udf") {
        let src = vendor.join("libcdio/lib/udf/");
        build.include(&src);
        build.files(UDF_SOURCES.iter().map(|s| src.join(s)));
    }

    build.includes(&includes);
    build.try_compile("cdio")?;

    Ok(includes)
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

/// Copy file at `from` to the given destination if not present,
/// creating any parent directories if needed.
fn cp(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), io::Error> {
    let to = to.as_ref();
    if to.exists() {
        return Ok(());
    }
    let parent = to
        .parent()
        .expect("`to` path should have a parent directory");
    fs::create_dir_all(parent)?;
    fs::copy(from, to).map(|_| ())
}

const CDIO_SOURCES: &[&str] = &[
    "_cdio_generic.c",
    "_cdio_stdio.c",
    "_cdio_stream.c",
    "abs_path.c",
    "aix.c",
    "audio.c",
    "cd_types.c",
    "cdio.c",
    "cdtext.c",
    "device.c",
    "disc.c",
    "ds.c",
    "FreeBSD/freebsd.c",
    "FreeBSD/freebsd_cam.c",
    "FreeBSD/freebsd_ioctl.c",
    "gnu_linux.c",
    "image/bincue.c",
    "image/cdrdao.c",
    "image_common.c",
    "image/nrg.c",
    "logging.c",
    "memory.c",
    "mmc/mmc.c",
    "mmc/mmc_hl_cmds.c",
    "mmc/mmc_ll_cmds.c",
    "mmc/mmc_util.c",
    "MSWindows/aspi32.c",
    "MSWindows/win32_ioctl.c",
    "MSWindows/win32.c",
    "netbsd.c",
    "osx.c",
    "read.c",
    "realpath.c",
    "sector.c",
    "solaris.c",
    "track.c",
    "utf8.c",
    "util.c",
];
const ISO9660_SOURCES: &[&str] = &["iso9660.c", "iso9660_fs.c", "rock.c", "xa.c"];
const UDF_SOURCES: &[&str] = &[
    "filemode.c",
    "udf.c",
    "udf_file.c",
    "udf_fs.c",
    "udf_time.c",
];

use clap::ValueEnum;
use clap_complete::Shell;
use clap_complete::generate_to;
use clap_mangen::Man;
use std::io::Write;
use std::{
    env,
    fs::File,
    io::Error,
    path::{Path, PathBuf},
};

mod app {
    pub mod types {
        include!("src/app/types.rs");
    }
}

use app::types::ConnectionString;

#[path = "src/cli/arguments.rs"]
mod arguments;

use arguments::{build_arguments, CliOptions};

struct PackageMeta {
    name: String,
    version: String,
    description: String,
    authors: String,
}

impl PackageMeta {
    pub fn try_new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            name: env::var("CARGO_PKG_NAME")?,
            version: env::var("CARGO_PKG_VERSION")?,
            description: env::var("CARGO_PKG_DESCRIPTION")?,
            authors: env::var("CARGO_PKG_AUTHORS")?,
        })
    }
}

fn build_shell_completion(
    outdir: &Path,
    package_meta: &PackageMeta,
) -> Result<(), Error> {
    let mut cmd = build_arguments(CliOptions::default());

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, &package_meta.name, outdir)?;
    }

    Ok(())
}

fn build_manpages(
    outdir: &Path,
    package_meta: &PackageMeta,
) -> Result<(), Error> {
    let app = build_arguments(CliOptions::default());

    let file = Path::new(&outdir).join(format!("{}.1", package_meta.name));
    let mut file = File::create(&file)?;

    Man::new(app).render(&mut file)?;

    Ok(())
}

fn build_control_file(
    outdir: &Path,
    package_meta: &PackageMeta,
) -> Result<(), Error> {
    let file_content = format!(
        "Package: {}\n\
         Version: {}\n\
         Architecture: amd64\n\
         Maintainer: {}\n\
         Description: {}\n\
         ",
        package_meta.name,
        package_meta.version,
        package_meta.authors,
        package_meta.description,
    );

    let mut file_path = PathBuf::from(outdir);
    file_path.push("control");

    let mut file = File::create(&file_path)?;

    file.write_all(file_content.as_bytes())?;

    file.flush()?;

    Ok(())
}

fn build_desktop_file(
    outdir: &Path,
    package_meta: &PackageMeta,
) -> Result<(), Error> {
    let file_content = format!(
        "[Desktop Entry]\n\
        Name={}\n\
        Comment={}\n\
        Exec=/usr/bin/{}\n\
        Icon={}\n\
        Terminal=true\n\
        Type=Application\n\
        Encoding=UTF-8\n\
        Categories=Network;Application;\n\
        Name[en_US]={}\n\
        ",
        package_meta.name,
        package_meta.description,
        package_meta.name,
        package_meta.name,
        package_meta.name,
    );

    let mut file_path = PathBuf::from(outdir);
    file_path.push(format!("{}.desktop", package_meta.name));

    let mut file = File::create(&file_path)?;

    file.write_all(file_content.as_bytes())?;

    file.flush()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/cli/arguments.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let package_meta = PackageMeta::try_new()?;

    let outdir = match env::var_os("OUT_DIR") {
        None => {
            println!(
                "cargo:warning=OUT_DIR variable was not found. Skipping creating assets"
            );
            return Ok(());
        }
        Some(outdir) => outdir,
    };

    let out_path = PathBuf::from(outdir);
    let base_path = out_path.ancestors().nth(4).unwrap().to_owned();

    let mut completions_path = base_path.clone();
    completions_path.push("completions");
    std::fs::create_dir_all(&completions_path).unwrap();

    build_shell_completion(&completions_path, &package_meta)?;

    let mut assets_path = base_path.clone();
    assets_path.push("assets");
    std::fs::create_dir_all(&assets_path).unwrap();

    build_manpages(&assets_path, &package_meta)?;
    build_control_file(&assets_path, &package_meta)?;
    build_desktop_file(&assets_path, &package_meta)?;

    Ok(())
}

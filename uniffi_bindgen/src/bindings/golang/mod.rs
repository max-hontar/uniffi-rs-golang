pub mod gen_golang;

use self::gen_golang::generate_bindings;
use crate::ComponentInterface;
use anyhow::{bail, Context, Result};
use camino::Utf8Path;
pub use gen_golang::Config;
use std::{fs::File, io::Write, process::Command};

/// The Golang bindings generated from a [`ComponentInterface`].
///
pub struct Bindings {
    /// The contents of the generated `.go` file, as a string.
    library: String,
    /// The contents of the generated `.h` file, as a string.
    header: String,
}

pub fn write_bindings(
    config: &Config,
    ci: &ComponentInterface,
    out_dir: &Utf8Path,
    _try_format_code: bool,
) -> Result<()> {
    let Bindings {
        header,
        library,
    } = generate_bindings(config, ci)?;

    let go_file = out_dir.join(format!("{}.go", ci.namespace()));
    let mut l = File::create(&go_file)?;
    write!(l, "{}", library)?;

    let h_file = out_dir.join(format!("{}.h", ci.namespace()));
    let mut h = File::create(&h_file)?;
    write!(h, "{}", header)?;

    Ok(())
}

pub fn run_script(out_dir: &Utf8Path, script_file: &Utf8Path) -> Result<()> {
    let go_proj = out_dir.join("go.mod");
    let mut go_proj = File::create(&go_proj)?;
    write!(
        go_proj,
        r#"
        module example

        go 1.19
        "#
    )?;

    let main = out_dir.join("main.go");
    std::fs::write(main, std::fs::read(script_file)?)?;

    let mut cmd = Command::new("go");
    cmd.current_dir(out_dir);
    cmd.arg("run");
    let status = cmd
        .spawn()
        .context("Failed to spawn `go` when running script")?
        .wait()
        .context("Failed to wait for `go` when running script")?;

    if !status.success() {
        bail!("running `go` failed")
    }

    Ok(())
}

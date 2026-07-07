use std::path::Path;
use std::{fs, io};

use clap::{Command, CommandFactory};
use clap_complete::Shell;

use crate::cli::Config;

fn generate_impl(
    shell: Shell,
    app: &mut Command,
    appname: &str,
    outdir: &Path,
    file: String,
) -> io::Result<()> {
    let destfile = outdir.join(file);
    let parent = destfile
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid completion path"))?;

    fs::create_dir_all(parent)?;
    let mut dest = fs::File::create(destfile)?;
    clap_complete::generate(shell, app, appname, &mut dest);
    Ok(())
}

pub fn generate(outdir: &Path) -> io::Result<()> {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};

    let appname = "richls";
    let mut app = Config::command().bin_name(appname);

    generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"))?;
    generate_impl(
        Elvish,
        &mut app,
        appname,
        outdir,
        format!("elvish/{appname}"),
    )?;
    generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"))?;
    generate_impl(
        PowerShell,
        &mut app,
        appname,
        outdir,
        format!("powershell/_{appname}.ps1"),
    )?;
    generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"))?;
    Ok(())
}

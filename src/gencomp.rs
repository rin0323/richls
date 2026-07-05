use std::path::Path;

use clap::{Command, CommandFactory};
use clap_complete::Shell;

use crate::cli::Config;

fn generate_impl(shell: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);

    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();

    if let Ok(mut dest) = std::fs::File::create(destfile) {
        clap_complete::generate(shell, app, appname, &mut dest);
    }
}

pub fn generate(outdir: &Path) {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};

    let appname = "richls";
    let mut app = Config::command().bin_name(appname);

    generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"));
    generate_impl(
        Elvish,
        &mut app,
        appname,
        outdir,
        format!("elvish/{appname}"),
    );
    generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"));
    generate_impl(
        PowerShell,
        &mut app,
        appname,
        outdir,
        format!("powershell/_{appname}.ps1"),
    );
    generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"));
}

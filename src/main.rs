mod constants;

use std::{
    process::{Command, Stdio, ExitStatus},
    env::{current_dir, args},
    error::Error,
};
use constants::*;
use colored::Colorize;

trait Trim {
    fn remove_after_last_backslash(self) -> Self;
}

impl Trim for String {
    fn remove_after_last_backslash(self) -> Self {
        if let Some(index) = self.rfind('/') {
            self[..index].to_string()
        } else {
            self
        }
    }
}

fn output_result(scp_out: ExitStatus, cwd: &str, target_dir: &str) {
    if scp_out.success() {
        println!(
"
================================================================================

scp {}: 
    + copied local directory      '{}' 
    + to cse account directory    '~/{}'

================================================================================
",
            "SUCCESS".green(),
            cwd,
            target_dir
        );
    } else {
        println!(
"
================================================================================
scp {}:
    + exit code: {:?}
================================================================================
",
            "FAILED".red(),
            scp_out.code()
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args= args().collect::<Vec<String>>();
    let cwd = current_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| "local directory can't be established!")?;

    if args.contains(&HELP_FLAG.to_string()) {
        print!("{HELP_MSG}");
        return Ok(());
    }
    if args.len() != NUM_ARGS {
        println!("{INVALID_ARGS_MSG}");
        return Ok(());
    }

    let target_dir = {
        if args.contains(&GUESS_FLAG.to_string()) {
            cwd.split("uni/")
                .into_iter()
                .nth(1)
                .ok_or("couldn't guess target directory :(")?
                .to_string()
                .remove_after_last_backslash()
        } else {
            args.remove(1)
        }
    };

    let scp_out = Command::new("scp")
        .arg("-r")
        .arg(&cwd)
        .arg(format!("{}:~/{}", REMOTE, target_dir))
        .stdout(Stdio::null())
        .status()?;
    
    output_result(scp_out, &cwd, &target_dir);

    Ok(())
}

use std::io::Error;
use std::process::Command;
use std::env;


fn main() -> Result<(), Error> {
    let status = if cfg!(unix) {
        Command::new("./mach").args(env::args()).status()?
    } else {
        Command::new("mach.bat").args(env::args()).status()?
    };
    assert!(status.success());
    Ok(())
}

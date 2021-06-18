use std::io::Error;
use std::process::Command;
use std::env;


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let status = if cfg!(unix) {
        Command::new("./mach").args(&args[2..]).status()?
    } else {
        Command::new("mach.bat").args(&args[2..]).status()?
    };
    assert!(status.success());
    Ok(())
}

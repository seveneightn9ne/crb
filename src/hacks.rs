use duct;
use std::process::Command;
use std::os::unix::process::CommandExt;

use errors::{CrbError, CrbResult};

pub fn recompile() -> CrbResult<()> {
    let result = try!(duct::sh("make")
        .null_stdout()
        .null_stderr()
        .run()
        .map_err(|e| CrbError::new(&format!("Error recompiling: {:?}", e))));
    match result.status {
        0 => Ok(()),
        _ => Err(CrbError::new(&format!("Recompiled with exit <{}>", result.status))),
    }
}

pub fn restart() -> CrbResult<()> {
    let e = Command::new("cargo").arg("run").arg("src/main.rs").exec();
    Err(CrbError::new(&format!("Error restarting: {:?}", e)))
}

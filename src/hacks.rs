use std::process;
use std::process::Command;
use std::os::unix::process::CommandExt;

use errors::{CrbError, CrbResult};

pub fn recompile() -> CrbResult<process::Output> {
    // duct::sh("make")
    //     .capture_stdout()
    //     .capture_stderr()
    //     .unchecked()
    //     .run()
    //     .map_err(|e| CrbError::new(&format!("Error recompiling: {:?}", e)))
    Command::new("make")
        .output()
        .map_err(|e| CrbError::new(&format!("Error recompiling: {:?}", e)))
}

/// Exec the new version.
pub fn restart() -> CrbResult<()> {
    let e = Command::new("cargo").arg("run").arg("src/main.rs").exec();
    Err(CrbError::new(&format!("Error restarting: {:?}", e)))
}

use std::{process::{Command, Child}, io::Error};

use crate::CONFIG;

pub fn load() -> Result<Child, Error> {
    let mut tsc = Command::new("pnpm");
    tsc.arg("tsc");

    if CONFIG.live_reloading {
        tsc.args(&["--watch", "--preserveWatchOutput"]);
    }

    tsc.spawn()
}
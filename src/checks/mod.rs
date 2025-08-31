//! Checks on the converted internal tree.
//! The checks are very important as it ensures that the
//! generated Rhai code is correct.

mod enter_check;

use enter_check::check_for_enter;

use crate::transpiler::InternalTree;
use colored::Colorize;
use log::error;

pub fn run_all_checks(tree: &InternalTree) -> Result<(), ()> {
    // CE == Check Error

    let mut checks_failed = 0;

    if let Err(e) = check_for_enter(tree) {
        error!("{} {}", "[CE01]".cyan(), e);
        checks_failed += 1
    }

    if checks_failed > 0 {
        if checks_failed == 1 {
            error!("1 check failed. Exiting.");
        } else {
            error!("{} checks failed. Exiting.", checks_failed);
        }

        Err(())
    } else {
        Ok(())
    }
}

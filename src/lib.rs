#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_getters;

pub use indicator_calculator::IndicatorCalculator;

use crate::git::GitCalculator;
use crate::git_logger::GitLogConfig;

pub mod git_file_history;
pub mod git_logger;
pub mod git;
pub mod git_file_future;
pub mod git_user_dictionary;
pub mod indicator_calculator;

pub fn scanner_by_years(git_years: u64) -> GitCalculator {
    let calculator = GitCalculator::new(
        GitLogConfig::default()
            .include_merges(true)
            .since_years(git_years),
        true,
    );

    calculator
}

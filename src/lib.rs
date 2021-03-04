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

use crate::git::GitCalculator;
use crate::git_logger::GitLogConfig;
use crate::toxicity_indicator_calculator::ToxicityIndicatorCalculator;

pub mod git_file_history;
pub mod git_logger;
pub mod git;
pub mod git_file_future;
pub mod git_user_dictionary;
pub mod toxicity_indicator_calculator;

pub fn scanner_by_years(git_years: u64) -> GitCalculator {
    let calculator = GitCalculator::new(
        GitLogConfig::default()
            .include_merges(true)
            .since_years(git_years),
        true,
    );

    calculator
}
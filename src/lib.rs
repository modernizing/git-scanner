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

pub mod git_file_history;
pub mod git_logger;
pub mod git;
pub mod git_file_future;
pub mod git_user_dictionary;
pub mod toxicity_indicator_calculator;

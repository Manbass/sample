pub mod db;
pub mod entities;
pub mod services;
pub mod use_cases;

use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("Some error")]
    SomeError,
}

pub type Result<T, E = crate::Error> = std::result::Result<T, E>;

fn main() {
    // match cmd, run worker for one of cmd
}

//! Tic-Tac-Toe Library
#![no_std]
#![allow(dead_code)]

extern crate alloc;

mod board;
mod error;
mod game;
mod history;
mod mark;

use core::fmt;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub use error::Error;

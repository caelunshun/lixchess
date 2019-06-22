#![feature(test)]
#[cfg(test)]
extern crate test;

#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_new;

pub mod board;
pub mod net;
pub mod proto;

/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0
*/
#![doc = include_str!("../README.md")]
mod charset;
mod error;
mod generator;
mod util;

pub use charset::SPECIAL_CHARS;
pub use error::Error;
pub use generator::{
  gen, PwdGen, PwdGenOptions, DEFAULT_PWDGEN_OPTIONS, MIN_LENGTH,
};

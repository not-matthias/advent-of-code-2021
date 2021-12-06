#![feature(bool_to_option)]
#![feature(slice_group_by)]

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[cfg(target_env = "DONT_COMPILE_THIS")] pub mod template;

aoc_lib! { year = 2021}

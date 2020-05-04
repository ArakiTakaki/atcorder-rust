#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use itertools::Itertools;
use proconio::{input, marker::*};
use superslice::*;
use whiteread::parse_line;

fn main() {
    let x: i64 = parse_line().unwrap();
    let mut num: i64 = 100;
    let mut count: usize = 0;
    loop {
        num = (num / 100) + num;
        count = count + 1;
        if num >= x {
            break;
        }
    }
    println!("{}", count);
}

#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use itertools::Itertools;
use proconio::{input, marker::*};
use superslice::*;
use whiteread::parse_line;
use std::collections::HashMap;

// 3 2
// 2
// 1 3
// 1
// 3
fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let mut v = vec![false; n];

    for _ in 0..k {
        let _: usize = parse_line().unwrap();
        let want_to_candy_list: Vec<usize> = parse_line().unwrap();
        for candy in want_to_candy_list {
            v[candy - 1] = true;
        }
    }
    let mut count: usize = 0;
    for is_get in v {
        count += if is_get { 0 } else { 1 };
    }
    println!("{}", count);
}

#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use itertools::Itertools;
use proconio::{input, marker::*};
use superslice::*;
use whiteread::parse_line;

fn main() {
    // let (n, m, q): (usize, usize, usize) = parse_line().unwrap();
    // let mut abcd_list: Vec<(usize, usize, usize, usize)> = Vec::with_capacity(50);
    // for _ in 0..q {
    //     abcd_list.push(parse_line().unwrap())
    // }
    // max_floor(n, m, q, abcd_list);
     dbg!(pow(3, 3));
     dbg!((3 as i32).pow(3));
}

// fn max_floor(n: usize, m: usize, q: usize, abcd_list: Vec<(usize, usize, usize, usize)>) -> i32 {
//     dbg!(n, m, q, abcd_list);
//     dbg!(pow(3, 3));
//     10
// }

fn pow(i: usize, num: usize) -> usize {
    private_pow(i, num, num)
}
// これができるんだったらクロージャーでいいんじゃないの
fn private_pow(i: usize, num: usize, current: usize ) -> usize {
    if i == 1 {
        return num;
    }
    private_pow(i - 1, num * current, current)
}
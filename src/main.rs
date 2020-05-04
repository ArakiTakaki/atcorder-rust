#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::HashMap;
use std::collections::*;
use superslice::*;
use whiteread::parse_line;

struct Obs {
    h: usize,
    is_toll: bool,
}

fn main() {
    // N 展望台の数 M ルートの数
    let (n, m): (usize, usize) = parse_line().unwrap();
    // H[N] 展望台の高さ
    let h_list: Vec<usize> = parse_line().unwrap();
    let mut obs_list: Vec<Obs> = vec![];

    // 構造体の初期化
    for idx in 0..n {
        obs_list.push(Obs {
            h: h_list[idx],
            is_toll: true,
        });
    }

    // 高さの算出
    for _ in 0..m {
        let (a, b): (usize, usize) = parse_line().unwrap();
        // 高さの解決
        fn calc(target: &Obs, comparison: &Obs) -> bool {
            if target.is_toll {
                return target.h > comparison.h;
            }
            false
        };
        obs_list[a - 1].is_toll = calc(&obs_list[a - 1], &obs_list[b - 1]);
        obs_list[b - 1].is_toll = calc(&obs_list[b - 1], &obs_list[a - 1]);
    }

    // ここをクロージャで記述したい。
    let mut count = 0;
    for obs in obs_list {
        if obs.is_toll {
            count = count + 1;
        }
    }
    print!("{}", count);
}

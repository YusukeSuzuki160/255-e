// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::Ordering;
use std::process;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                },
                Ordering::Greater | Ordering::Equal => {
                    high = mid;
                },
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                },
                Ordering::Greater => {
                    high = mid;
                },
            }
        }
        low
    }
}
// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n - 1],
        x: [i64; m],
    }
    let mut zig_sum = Vec::new();
    if n == 2 {
        for i in 0..m {
            for j in i..m {
                if x[i] + x[j] == s[0] {
                    println!("2");
                    process::exit(0);
                }
            }
        }
        println!("1");
        process::exit(0);
    }
    zig_sum.push(s[0]);
    for i in 1..(n - 1) {
        zig_sum.push(s[i] - zig_sum[i - 1]);
    }
    let mut good_a = Vec::new();
    for i in 0..(n - 1) {
        for j in 0..m {
            if i % 2 == 0 {
                good_a.push(x[j] - zig_sum[i]);
            } else {
                good_a.push(zig_sum[i] - x[j]);
            }
        }
    }
    good_a.sort();
    let mut ans = 0;
    for i in 0..good_a.len() {
        let lower_index = good_a.lower_bound(&good_a[i]);
        let upper_index = good_a.upper_bound(&good_a[i]);
        let same_num = upper_index - lower_index;
        if ans < same_num {
            ans = same_num;
        }
    }
    println!("{}", ans);
}

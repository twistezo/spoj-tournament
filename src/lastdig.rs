use std::io;
use std::io::prelude::*;

pub fn run() {
    for l in io::stdin().lock().lines().skip(1) {
        let n = l
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        println!("{}", last_digit(n[0], n[1]));
    }
}

fn last_digit(a: u32, mut b: u32) -> u32 {
    let q;
    let mut w = 1;
    if b != 0 {
        q = b % 4;
        if q == 0 {
            b = 4;
        } else {
            b = q;
        }
        for _ in 0..b {
            w *= a
        }
        return w % 10;
    } else {
        return 1;
    }
}

use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut set = Vec::new();
    let mut set_nxt = Vec::new();
    set.push(Vec::<u64>::new());
    for &ai in &a {
        set_nxt.clear();
        for g in &set {
            for i in 0..g.len() {
                let mut new_v = g.clone();
                new_v[i] += ai;
                set_nxt.push(new_v);
            }
            let mut new_v = g.clone();
            new_v.push(ai);
            set_nxt.push(new_v);
        }
        set.clone_from(&set_nxt);
    }
    
    let mut ans_set = HashSet::new();
    for g in &set {
        ans_set.insert(g.iter().copied().reduce(|acc, x| acc ^ x).unwrap());
    }
    println!("{}", ans_set.len());
}

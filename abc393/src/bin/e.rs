use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mx = 1_000_000;
    let mut divisors = vec![vec![1]; mx + 1];
    for i in 2..=mx {
        let mut j = i;
        while j <= mx {
            divisors[j].push(i);
            j += i;
        }
    }
    eprintln!("{}", divisors.iter().map(|x| x.len()).max().unwrap());
    eprintln!("{divisors:?}");

    let mut div_cnt = vec![0usize; mx + 1];
    for &ai in &a {
        for &d in &divisors[ai] {
            div_cnt[d] += 1;
        }
    }

    // for &ai in &a {
    //     for &d in divisors[ai].iter().rev() {
    //         if div_cnt[d] >= k {
    //             println!("{}", d);
    //             break;
    //         }
    //     }
    // }
}

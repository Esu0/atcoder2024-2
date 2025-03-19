use proconio::input;

fn main() {
    input! {
        k: u64,
    }
    let mut divs = vec![];
    let mut i = 1;
    while i * i < k {
        if k % i == 0 {
            divs.push(i);
            divs.push(k / i);
        }
        i += 1;
    }
    if i * i == k {
        divs.push(i);
    }
    divs.sort_unstable();
    let mut ans = 0usize;
    for (i, &a) in divs.iter().enumerate() {
        let k0 = k / a;
        for &b in &divs[i..] {
            if k0 % b == 0 {
                let c = k0 / b;
                if c >= b {
                    ans += 1;
                }
            }
        }
    }
    println!("{ans}");
}

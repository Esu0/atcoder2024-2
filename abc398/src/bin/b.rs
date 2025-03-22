use proconio::input;

fn main() {
    input! {
        a: [u32; 7],
    }

    let mut cnt = [0; 13];
    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                for l in k + 1..6 {
                    for m in l + 1..7 {
                        let arr = [a[i], a[j], a[k], a[l], a[m]];
                        cnt.fill(0);
                        for &arri in &arr {
                            cnt[arri as usize - 1] += 1;
                        }
                        if cnt.iter().any(|&ci| ci == 2) && cnt.iter().any(|&ci| ci == 3) {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}

use proconio::input;

fn main() {
    input! {
        mut n: proconio::marker::Bytes,
        k: usize,
    }

    n.reverse();
    n.iter_mut().for_each(|ni| *ni -= b'0');
    n.resize(20, 0);

    for _ in 0..k {
        let mut num = 0;
        for i in 0..20 {
            num += (n[i] as u64) << (i * 3);
        }

        for i in 0..20 {
            n[i] = (num % 9) as u8;
            if n[i] == 8 {
                n[i] = 5;
            }
            num /= 9;
        }
    }

    if n.iter().all(|&x| x == 0) {
        println!("0");
        return;
    }
    n.iter()
        .rev()
        .skip_while(|&&x| x == 0)
        .for_each(|&c| print!("{c}"));
    println!();
}

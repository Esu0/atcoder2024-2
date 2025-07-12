use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    'next: for _ in 0..t {
        input! {
            n: usize,
            mut a: [i64; n],
        }

        let p = a[0];
        let (mut cntp, mut cntn) = (0usize, 0);
        for &ai in &a {
            if ai == p {
                cntp += 1;
            }
            if ai == -p {
                cntn += 1;
            }
        }
        if cntp > cntn {
            (cntp, cntn) = (cntn, cntp);
        }
        if cntp == n / 2 && cntn == n - n / 2 {
            println!("Yes");
            continue;
        }
        a.sort_unstable_by_key(|&x| x.abs());

        let rp = a[1];
        let rq = a[0];
        for i in 2..n {
            // must be `rp / rq == a[i] / a[i - 1]`
            if rp * a[i - 1] != a[i] * rq {
                println!("No");
                continue 'next;
            }
        }
        println!("Yes");
    }
}

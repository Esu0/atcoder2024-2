use proconio::input;

fn solve(buf: &mut [usize]) {
    if buf.len() == 1 {
        return;
    }
    let m = buf.len() / 2;
    let (buf1, buf2) = buf.split_at_mut(m);
    solve(buf1);
    solve(buf2);
    if buf1[0] > buf2[0] {
        buf2.swap_with_slice(buf1);
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut p: [usize; 1 << n],
        }

        solve(&mut p);
        print!("{}", p[0]);
        for &pi in &p[1..] {
            print!(" {pi}");
        }
        println!();
    }
}

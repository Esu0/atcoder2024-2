use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; 2 * n],
        }

        let mut pos = vec![(usize::MAX, 0); n];
        for (i, &ai) in a.iter().enumerate() {
            if pos[ai - 1].0 == usize::MAX {
                pos[ai - 1].0 = i;
            } else {
                pos[ai - 1].1 = i;
            }
        }

        let mut ans = 0;
        for i in 0..2 * n - 1 {
            if pos[a[i] - 1].0 == i
                && pos[a[i + 1] - 1].0 == i + 1
                && pos[a[i] - 1].1.abs_diff(pos[a[i + 1] - 1].1) == 1
                && pos[a[i + 1] - 1].1 != pos[a[i + 1] - 1].0 + 1
            {
                ans += 1;
            }
        }
        println!("{ans}");
    }
}

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![vec![0; n]; n];
    let mut set = (1..=n * n).collect::<HashSet<_>>();
    if n % 2 == 0 {
        for (i, row) in ans.iter_mut().enumerate() {
            set.remove(&(3 * i + 4));
            set.remove(&(3 * i + 5));
            row[n / 2 - 1] = 3 * i + 4;
            row[n / 2] = 3 * i + 5;
            if i % 2 == 1 {
                row.swap(n / 2 - 1, n / 2);
            }
            // eprintln!("{} {}", row[n / 2], row[n / 2 + 1]);
        }
        let mut iter = set.iter().filter(|&&x| x % 2 == 0).copied();
        for row in &mut ans {
            for cell in &mut row[..n / 2 - 1] {
                *cell = iter.next().unwrap();
            }
        }
        assert!(iter.next().is_none());

        let mut iter = set.iter().filter(|&&x| x % 2 == 1).copied();
        for row in &mut ans {
            for cell in &mut row[n / 2 + 1..] {
                *cell = iter.next().unwrap();
            }
        }
        assert!(iter.next().is_none());
    } else if n == 3 {
        ans = vec![
            vec![8, 7, 2],
            vec![1, 5, 4],
            vec![9, 3, 6],
        ];
    } else {
        set.remove(&4);
        set.remove(&5);
        set.remove(&7);
        set.remove(&8);
        ans[0][n / 2] = 5;
        ans[0][n / 2 + 1] = 4;
        ans[1][n / 2] = 7;
        ans[1][n / 2 + 1] = 8;

        let mut i = 0;
        for row in ans[2..n / 2].iter_mut() {
            assert!(set.remove(&(3 * i + 16)));
            assert!(set.remove(&(3 * i + 17)));
            row[n / 2] = 3 * i + 16;
            row[n / 2 + 1] = 3 * i + 17;
            if i % 2 == 0 {
                row.swap(n / 2, n / 2 + 1);
            }
            i += 1;
        }
        assert!(set.remove(&11));
        assert!(set.remove(&14));
        assert!(set.remove(&10));
        assert!(set.remove(&15));
        ans[n / 2][n / 2] = 11;
        ans[n / 2][n / 2 + 1] = 14;
        ans[n / 2 + 1][n / 2] = 10;
        ans[n / 2 + 1][n / 2 - 1] = 15;

        for row in &mut ans[n / 2 + 2..] {
            assert!(set.remove(&(3 * i + 16)));
            assert!(set.remove(&(3 * i + 17)));
            row[n / 2 - 1] = 3 * i + 16;
            row[n / 2] = 3 * i + 17;
            if i % 2 == 0 {
                row.swap(n / 2 - 1, n / 2);
            }
            i += 1;
        }

        let mut iter0 = set.iter().filter(|&&x| x % 2 == 0).copied();
        let mut iter1 = set.iter().filter(|&&x| x % 2 == 1).copied();
        for row in &mut ans[0..=n / 2] {
            for cell in &mut row[0..n / 2] {
                *cell = iter1.next().unwrap();
            }
            for cell in &mut row[n / 2 + 2..] {
                *cell = iter0.next().unwrap();
            }
        }
        for row in &mut ans[n / 2 + 1..] {
            for cell in &mut row[0..n / 2 - 1] {
                *cell = iter1.next().unwrap();
            }
            for cell in &mut row[n / 2 + 1..] {
                *cell = iter0.next().unwrap();
            }
        }
        assert!(iter0.next().is_none());
        assert!(iter1.next().is_none());
    }

    for row in &ans {
        print!("{}", row[0]);
        for &cell in &row[1..] {
            print!(" {cell}");
        }
        println!();
    }
}

use proconio::input;

fn partial_matching_table<T: Eq>(s: &[T]) -> Vec<usize> {
    let mut table = Vec::with_capacity(s.len() + 1);
    table.push(usize::MAX);
    if s.is_empty() {
        return table;
    }

    let mut j = usize::MAX;
    for si in s {
        while j != usize::MAX && &s[j] != si {
            j = table[j];
        }
        j = j.wrapping_add(1);
        table.push(j);
    }
    table
}

fn kmp(s1: &[u8], s2: &[u8], pmt: &[usize]) -> usize {
    let mut j = 0;
    for &s1i in s1 {
        if j == s2.len() {
            return j;
        }
        while j != usize::MAX && s1i != s2[j] {
            j = pmt[j];
        }
        j = j.wrapping_add(1);
    }
    j
}

fn main() {
    input! {
        n: usize,
        mut s: [proconio::marker::Bytes; n],
    }

    let mut pmts = Vec::with_capacity(n);
    for si in &s {
        pmts.push(partial_matching_table(si));
    }

    let mut avail = vec![true; n];
    let mut max_match = Vec::with_capacity(n);
    for (i, si) in s.iter().enumerate() {
        let pmt = &pmts[i];
        let mut new_row = Vec::with_capacity(n);
        for (j, sj) in s.iter().enumerate() {
            let match_count = kmp(sj, si, pmt);
            if si.len() == match_count {
                if si.len() == sj.len() {
                    if i > j {
                        avail[i] = false;
                    }
                } else {
                    avail[i] = false;
                }
            }
            new_row.push(match_count);
        }
        max_match.push(new_row);
    }
    let mut itr = avail.iter();
    max_match.retain(|_| *itr.next().unwrap());
    max_match.iter_mut().for_each(|row| {
        let mut itr = avail.iter();
        row.retain(|_| *itr.next().unwrap());
    });
    let mut itr = avail.iter();
    s.retain(|_| *itr.next().unwrap());

    // eprintln!("{s:?}");
    // eprintln!("{max_match:?}");

    let n = s.len();
    // 巡回セールスマン問題

    let mut dp = vec![vec![usize::MAX; n]; 1 << n];

    for i in 0..n {
        dp[1 << i][i] = s[i].len();
    }

    for set in 1..1 << n {
        for i in 0..n {
            if set & (1 << i) == 0 {
                continue;
            }
            for j in 0..n {
                if set & (1 << j) != 0 {
                    continue;
                }
                let set2 = set | (1 << j);
                dp[set2][j] = dp[set2][j].min(dp[set][i].saturating_add(s[j].len() - max_match[j][i]));
            }
        }
    }
    let ans = dp[(1 << n) - 1].iter().copied().min().unwrap();
    println!("{ans}");
}

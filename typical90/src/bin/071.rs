use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];
    let mut indegs = vec![0usize; n];
    for &(ai, bi) in &ab {
        g[ai - 1].push(bi - 1);
        indegs[bi - 1] += 1;
    }

    let mut lst = Vec::with_capacity(n);
    for (i, &indeg) in indegs.iter().enumerate() {
        if indeg == 0 {
            lst.push(i);
        }
    }

    let mut ans = vec![Vec::with_capacity(n); k];
    enum Log {
        Push,
        Remove(usize, usize),
        Dec(usize),
    }

    let mut log = vec![];
    while let Some(u) = lst.pop() {
        log.push(Log::Remove(lst.len(), u));
        ans[0].push(u + 1);
        for &v in &g[u] {
            indegs[v] -= 1;
            log.push(Log::Dec(v));
            if indegs[v] == 0 {
                lst.push(v);
                log.push(Log::Push);
            }
        }
    }

    if ans[0].len() != n {
        println!("-1");
        return;
    }

    for i in 1..k {
        let (u, v) = loop {
            let Some(l) = log.pop() else {
                println!("-1");
                return;
            };
            match l {
                Log::Dec(v) => indegs[v] += 1,
                Log::Push => {
                    lst.pop();
                }
                Log::Remove(i, v) => {
                    if i == 0 {
                        lst.insert(0, v);
                    } else {
                        let u = lst[i - 1];
                        lst[i - 1] = v;
                        log.push(Log::Remove(i - 1, u));
                        break (u, v);
                    }
                }
            }
        };
        let mut j = 0;
        while ans[i - 1][j] != v + 1 {
            let tmp = ans[i - 1][j];
            ans[i].push(tmp);
            j += 1;
        }
        ans[i].push(u + 1);
        for &v in &g[u] {
            indegs[v] -= 1;
            log.push(Log::Dec(v));
            if indegs[v] == 0 {
                lst.push(v);
                log.push(Log::Push);
            }
        }
        while let Some(u) = lst.pop() {
            log.push(Log::Remove(lst.len(), u));
            ans[i].push(u + 1);
            for &v in &g[u] {
                indegs[v] -= 1;
                log.push(Log::Dec(v));
                if indegs[v] == 0 {
                    lst.push(v);
                    log.push(Log::Push);
                }
            }
        }
        assert_eq!(ans[i].len(), n);
    }

    let mut rev = vec![usize::MAX; n];

    for row in &ans {
        for (i, &v) in row.iter().enumerate() {
            rev[v - 1] = i;
        }
        for &(ai, bi) in &ab {
            assert!(rev[ai - 1] < rev[bi - 1]);
        }
        print!("{}", row[0]);
        for &v in &row[1..] {
            print!(" {}", v);
        }
        println!();
    }
}

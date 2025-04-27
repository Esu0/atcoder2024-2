use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Bytes,
        t: proconio::marker::Bytes,
    }

    let mut g = [usize::MAX; 26];
    for i in 0..n {
        let si = (s[i] - b'a') as usize;
        let ti = (t[i] - b'a') as usize;
        if g[si] == usize::MAX {
            g[si] = ti;
        } else if g[si] != ti {
            println!("-1");
            return;
        }
    }

    let mut vis = [false; 26];
    let mut cnt = 0;
    let mut stack = vec![];
    for u in 0..26 {
        if vis[u] {
            continue;
        }
        let mut v = u;
        stack.clear();
        loop {
            if stack.len() > 30 {
                break;
            }
            if v == usize::MAX {
                break;
            }
            stack.push(v);
            v = g[v];
            if v == u {
                if stack.len() > 1 {
                    cnt += 1;
                    while let Some(v) = stack.pop() {
                        vis[v] = true;
                    }
                }
                break;
            }
        }
    }

    let t = g.iter().enumerate().filter(|&(i, &x)| x != i && x != usize::MAX).count();
    let mut s = [false; 26];
    for &gi in &g {
        if gi != usize::MAX {
            s[gi] = true;
        }
    }
    if cnt >= 1 && s == [true; 26] {
        println!("-1");
    } else {
        println!("{}", t + cnt);
    }
    // eprintln!("{cnt}");
}

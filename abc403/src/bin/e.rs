use proconio::input;

#[derive(Debug, Clone, Copy)]
struct Node {
    child: [usize; 26],
    // parent: usize,
    is_dead: bool,
    y_cnt: usize,
}

fn main() {
    input! {
        q: usize,
    }

    let mut trie = vec![Node {
        child: [usize::MAX; 26],
        // parent: usize::MAX,
        is_dead: false,
        y_cnt: 0,
    }];
    let mut ans = 0;
    let mut stack = Vec::new();
    use std::fmt::Write;
    let mut ans_s = String::new();
    for _ in 0..q {
        input! { t: u8, s: proconio::marker::Bytes }
        let mut cur = 0;
        for &si in &s {
            if trie[cur].child[(si - b'a') as usize] == usize::MAX {
                let id = trie.len();
                trie.push(Node {
                    child: [usize::MAX; 26],
                    // parent: cur,
                    is_dead: trie[cur].is_dead,
                    y_cnt: 0,
                });
                trie[cur].child[(si - b'a') as usize] = id;
            }
            cur = trie[cur].child[(si - b'a') as usize];
        }
        if t == 1 {
            if !trie[cur].is_dead {
                stack.push(cur);
                while let Some(cur) = stack.pop() {
                    ans -= trie[cur].y_cnt;
                    trie[cur].is_dead = true;
                    for nxt in trie[cur].child {
                        if nxt != usize::MAX && !trie[nxt].is_dead {
                            stack.push(nxt);
                        }
                    }
                }
            }
        } else {
            trie[cur].y_cnt += 1;
            if !trie[cur].is_dead {
                ans += 1;
            }
        }
        writeln!(&mut ans_s, "{ans}").unwrap();
    }
    print!("{ans_s}");
}

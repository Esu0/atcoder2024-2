use proconio::input;
type MInt = util::ModInt<998244353>;

struct Node {
    next: [usize; 2],
    value: MInt,
    flag: bool,
    count: usize,
}

impl Node {
    fn new() -> Self {
        Self {
            next: [usize::MAX; 2],
            value: MInt::new(0),
            flag: false,
            count: 0,
        }
    }
}
fn main() {
    input! {
        n: usize,
    }

    let mut trie = vec![Node::new()];

    let mut stack = vec![];
    for _ in 0..n {
        input! { s: proconio::marker::Bytes }
        let mut cur = 0;
        for &si in &s {
            let ind = (si - b'A') as usize;
            if trie[cur].next[ind] == usize::MAX {
                let i = trie.len();
                trie[cur].next[ind] = i;
                trie.push(Node::new());
            }
            stack.push(cur);
            cur = trie[cur].next[ind];
        }

        trie[cur].flag = true;
        let p2 = MInt::new(2).pow(trie[cur].count as _);
        trie[cur].value += p2;
        while let Some(u) = stack.pop() {
            trie[u].count += 1;
            let mut new_val = if trie[u].flag {
                MInt::new(2).pow(trie[u].count as u32)
            } else {
                MInt::new(0)
            };
            let mut prod = MInt::new(1);
            for i in 0..2 {
                prod *= if trie[u].next[i] == usize::MAX {
                    MInt::new(0)
                } else {
                    let ind = trie[u].next[i];
                    trie[ind].value
                };
            }
            new_val += prod;
            trie[u].value = new_val;
        }

        println!("{}", trie[0].value);
    }
}

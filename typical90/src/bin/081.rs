use proconio::input;

// range max, range add
struct LazySegtree {
    arr: Vec<usize>,
    lazy: Vec<usize>,
}

impl LazySegtree {
    pub fn new(itr: impl IntoIterator<Item = usize>) -> Self {
        let arr = itr.into_iter().collect::<Vec<_>>();
        let length = arr.len().next_power_of_two();
        let mut arr = (0..length).map(|_| 0).chain(arr).collect::<Vec<_>>();
        arr.resize(length * 2, 0);
        for i in (1..arr.len() / 2).rev() {
            arr[i] = arr[i * 2].max(arr[i * 2 + 1]);
        }
        Self {
            lazy: vec![0; arr.len() / 2],
            arr,
        }
    }

    pub fn apply(&mut self, l: usize, r: usize, val: usize) {
        if l == r {
            return;
        }
        let length = self.lazy.len();
        self.apply_rec(l, r, 1, 0, length, val);
    }

    fn push(&mut self, i: usize) {
        let v = self.lazy[i];
        self.lazy[i] = 0;
        self.apply_one(i * 2, v);
        self.apply_one(i * 2 + 1, v);
    }

    fn eval(&mut self, i: usize) {
        self.arr[i] = self.arr[i * 2].max(self.arr[i * 2 + 1]);
    }

    fn apply_one(&mut self, i: usize, val: usize) {
        if i < self.lazy.len() {
            self.lazy[i] = self.lazy[i].wrapping_add(val);
        }
        self.arr[i] = self.arr[i].wrapping_add(val);
    }

    fn apply_rec(&mut self, l: usize, r: usize, u: usize, a: usize, b: usize, val: usize) {
        if r <= a || b <= l {
            return;
        }
        if l <= a && b <= r {
            self.apply_one(u, val);
            return;
        }
        let m = a + (b - a) / 2;
        self.push(u);
        self.apply_rec(l, r, 2 * u, a, m, val);
        self.apply_rec(l, r, 2 * u + 1, m, b, val);
        self.eval(u);
    }

    pub fn query(&mut self, l: usize, r: usize) -> usize {
        let length = self.lazy.len();
        self.query_rec(l, r, 1, 0, length)
    }

    fn query_rec(&mut self, l: usize, r: usize, u: usize, a: usize, b: usize) -> usize {
        if r <= a || b <= l {
            return 0;
        }
        if l <= a && b <= r {
            return self.arr[u];
        }
        let m = a + (b - a) / 2;
        self.push(u);
        let q1 = self.query_rec(l, r, 2 * u, a, m);
        let q2 = self.query_rec(l, r, 2 * u + 1, m, b);
        q1.max(q2)
    }

    // pub fn push_all(&mut self) {
    //     for i in 1..self.lazy.len() {
    //         self.push(i);
    //     }
    // }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut ws = vec![vec![]; 5002];
    for &(ai, bi) in &ab {
        ws[ai].push(bi);
    }
    let mut segtree = LazySegtree::new((0..5002).map(|_| 0));

    for i in 1..=k + 1 {
        for &bj in &ws[i] {
            segtree.apply(bj.saturating_sub(k), bj + 1, 1);
        }
    }
    let mut ans = segtree.query(0, 5002);
    for i in k + 2..5002 {
        let pi = i - k - 1;
        for &bj in &ws[i] {
            segtree.apply(bj.saturating_sub(k), bj + 1, 1);
        }
        for &bj in &ws[pi] {
            segtree.apply(bj.saturating_sub(k), bj + 1, usize::MAX);
        }
        ans = ans.max(segtree.query(0, 5002));
    }
    println!("{ans}");
}

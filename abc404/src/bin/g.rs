use proconio::input;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    weight: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![usize::MAX; n],
            size: vec![1; n],
            weight: vec![0; n],
        }
    }

    fn root(&mut self, u: usize) -> usize {
        let p = self.parent[u];
        if p == usize::MAX {
            u
        } else {
            let r = self.root(p);
            self.weight[u] += self.weight[p];
            self.parent[u] = r;
            r
        }
    }

    fn weight(&mut self, u: usize) -> i64 {
        self.root(u);
        self.weight[u]
    }

    fn diff(&mut self, u: usize, v: usize) -> i64 {
        self.weight(u) - self.weight(v)
    }

    fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn unite(&mut self, u: usize, v: usize, weight: i64) -> bool {
        let mut w = weight + self.diff(u, v);
        let mut ru = self.root(u);
        let mut rv = self.root(v);
        if ru == rv {
            return false;
        }

        if self.size[ru] < self.size[rv] {
            (ru, rv) = (rv, ru);
            w = -w;
        }

        self.parent[rv] = ru;
        self.size[ru] += self.size[rv];

        self.weight[rv] = w;
        true
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        lrs: [(usize, usize, i64); m],
    }

    
    // eprintln!("{}", uf.diff(0, 2));
}

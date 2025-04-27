use proconio::input;

struct Ctx {
    stack: Vec<usize>,
    sets: Vec<Vec<usize>>,
    ord: Box<[usize]>,
    low: Box<[usize]>,
    cur_ord: usize,
}

impl Ctx {
    fn new(n: usize) -> Self {
        let init = (0..n).map(|_| usize::MAX).collect::<Box<[_]>>();
        Self {
            stack: Vec::with_capacity(n),
            sets: Vec::with_capacity(n),
            ord: init.clone(),
            low: init,
            cur_ord: 0,
        }
    }

    fn dfs(&mut self, g: &[Vec<usize>]) {
        for u in 0..g.len() {
            if self.ord[u] == usize::MAX {
                dfs(g, u, self);
            }
        }
    }
}

fn dfs(g: &[Vec<usize>], u: usize, ctx: &mut Ctx) {
    ctx.stack.push(u);
    ctx.ord[u] = ctx.cur_ord;
    ctx.low[u] = ctx.cur_ord;
    ctx.cur_ord += 1;

    for &v in &g[u] {
        if ctx.ord[v] == usize::MAX {
            dfs(g, v, ctx);
            ctx.low[u] = ctx.low[u].min(ctx.low[v]);
        } else {
            ctx.low[u] = ctx.low[u].min(ctx.ord[v]);
        }
    }

    if ctx.ord[u] == ctx.low[u] {
        let mut s = vec![];
        loop {
            let v = ctx.stack.pop().unwrap();
            s.push(v);
            ctx.ord[v] = usize::MAX - 1;
            if u == v {
                break;
            }
        }
        ctx.sets.push(s);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
    }

    let mut ctx = Ctx::new(n);
    ctx.dfs(&g);

    let mut ans = 0;
    for si in &ctx.sets {
        ans += si.len() * (si.len() - 1) / 2;
    }
    println!("{ans}");
}

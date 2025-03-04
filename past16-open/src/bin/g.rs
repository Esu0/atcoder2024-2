use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n * 3],
    }

    let mut gs = vec![Vec::<Vec<u32>>::new()];
    let mut gs_new = vec![];
    for &ai in &a {
        gs_new.clear();
        for g in &gs {
            for (j, gj) in g.iter().enumerate() {
                if gj.len() < 3 {
                    let mut g_new = g.clone();
                    g_new[j].push(ai);
                    gs_new.push(g_new);
                }
            }
            if g.len() < n {
                let mut g_new = g.clone();
                g_new.push(vec![ai]);
                gs_new.push(g_new);
            }
        }
        gs.clone_from(&gs_new);
    }
    let ans = gs
        .iter()
        .filter(|g| {
            g.iter()
                .all(|gi| gi[0] + gi[1] > gi[2] && gi[0] + gi[2] > gi[1] && gi[1] + gi[2] > gi[0])
        })
        .count();
    println!("{ans}");
}

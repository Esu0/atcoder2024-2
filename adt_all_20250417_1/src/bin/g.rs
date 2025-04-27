use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xyc: [(usize, usize, char); m],
    }

    xyc.sort_unstable();
    let mut i = 0;
    let mut mn = n;
    while i < m {
        let xi = xyc[i].0;
        let mut mxb = 0;
        let mut mnw = n;
        while i < m && xyc[i].0 == xi {
            let (_, yi, ci) = xyc[i];
            // let xi = xi - 1;
            let yi = yi - 1;
            if ci == 'B' {
                mxb = mxb.max(yi);
            } else {
                mnw = mnw.min(yi);
            }
            i += 1;
        }
        if mnw < mxb || mxb >= mn {
            println!("No");
            return;
        }
        mn = mn.min(mnw);
    }
    println!("Yes");
}

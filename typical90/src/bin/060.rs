use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    enum Op {
        Push,
        Update(usize, u32),
    }

    let mut dp1 = Vec::<u32>::with_capacity(n);
    let mut dp2 = Vec::<u32>::with_capacity(n);
    let mut ops = Vec::with_capacity(n);

    for &ai in a.iter().rev() {
        let j = dp2.partition_point(|&x| x < ai);
        if j == dp2.len() {
            dp2.push(ai);
            ops.push(Op::Push);
        } else {
            ops.push(Op::Update(j, dp2[j]));
            dp2[j] = ai;
        }
    }

    let mut ans = 0;
    for &ai in &a {
        match ops.pop().unwrap() {
            Op::Push => {
                dp2.pop();
            }
            Op::Update(j, v) => {
                dp2[j] = v;
            }
        };
        let j = dp1.partition_point(|&x| x < ai);
        let k = dp2.partition_point(|&x| x < ai);
        ans = ans.max(j + k + 1);
        if j == dp1.len() {
            dp1.push(ai);
        } else {
            dp1[j] = ai;
        }
    }

    println!("{ans}");
}

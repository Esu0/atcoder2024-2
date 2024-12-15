use proconio::input;

fn main() {
    input! {
        n: usize,
        r: i32,
        da: [(u8, i32); n],
    }

    let mut rate = r;
    let ranges = [[1600, 2799], [1200, 2399]];

    for &(di, ai) in &da {
        let di = (di - 1) as usize;
        if (ranges[di][0]..=ranges[di][1]).contains(&rate) {
            rate += ai;
        }
    }
    println!("{rate}");
}

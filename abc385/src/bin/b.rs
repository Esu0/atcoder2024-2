use proconio::input;

fn main() {
    input! {
        h: usize,
        _w: usize,
        mut x: usize,
        mut y: usize,
        mut s: [proconio::marker::Bytes; h],
        t: proconio::marker::Bytes,
    }

    x -= 1;
    y -= 1;
    let mut dxy = [(usize::MAX / 2, usize::MAX / 2); 256];
    dxy[b'U' as usize] = (usize::MAX, 0);
    dxy[b'D' as usize] = (1, 0);
    dxy[b'L' as usize] = (0, usize::MAX);
    dxy[b'R' as usize] = (0, 1);

    let mut ans = 0;
    for &ti in &t {
        let nx = x.wrapping_add(dxy[ti as usize].0);
        let ny = y.wrapping_add(dxy[ti as usize].1);
        if s[nx][ny] == b'#' {
            continue;
        }
        x = nx;
        y = ny;
        if s[nx][ny] == b'@' {
            ans += 1;
            s[nx][ny] = 0;
        }
    }

    println!("{} {} {ans}", x + 1, y + 1);
}

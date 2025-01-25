use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [marker::Bytes; h],
    }

    let blacks = s.iter().enumerate().flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c))).filter(|&(_, _, c)| c == b'#').map(|(i, j, _)| (i, j));

    let mxi = blacks.clone().map(|(i, _)| i).max().unwrap();
    let mni = blacks.clone().map(|(i, _)| i).min().unwrap();
    let mxj = blacks.clone().map(|(_, j)| j).max().unwrap();
    let mnj = blacks.clone().map(|(_, j)| j).min().unwrap();

    eprintln!("{mxi} {mni} {mxj} {mnj}");
    for i in 0..h {
        for j in 0..w {
            if (mni..=mxi).contains(&i) && (mnj..=mxj).contains(&j) && s[i][j] == b'.' {
                    println!("No");
                    return;
            }
        }
    }
    println!("Yes");
}

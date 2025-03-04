use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u8; n],
    }

    let count1 = a.iter().filter(|&&x| x == 1).count();
    if n % 4 == 0 {
        println!("Yes");
    } else if n % 4 == 1 || n % 4 == 3 {
        if count1 > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if count1 == 0 {
            println!("No");
            return;
        }
        let mut pos1 = vec![];
        for i in 0..n {
            if a[i] == 1 {
                pos1.push(i);
            }
        }
        for i in 0..pos1.len() - 1 {
            if (pos1[i + 1] - pos1[i]) % 2 != 0 {
                println!("Yes");
                return;
            }
        }
        println!("No");
    }
}

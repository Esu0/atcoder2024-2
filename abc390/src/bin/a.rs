use proconio::input;

fn main() {
    input! {
        a: [u32; 5],
    }

    for i in 0..4 {
        let mut arr = a.clone();
        arr.swap(i, i + 1);
        if arr[..] == [1, 2, 3, 4, 5] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

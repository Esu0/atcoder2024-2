use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    for e in 1..=13 {
        let mut cards = [a, b, c, d, e];
        cards.sort_unstable();
        if cards[0] == cards[1] && cards[1] == cards[2] && cards[2] != cards[3] && cards[3] == cards[4] {
            println!("Yes");
            return;
        }
        if cards[0] == cards[1] && cards[1] != cards[2] && cards[2] == cards[3] && cards[3] == cards[4] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

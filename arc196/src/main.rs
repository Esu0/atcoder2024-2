fn main() {
    for i in 0..1usize << 10 {
        if i.count_ones() == 5 {
            println!("{i:010b}");
        }
    }
}
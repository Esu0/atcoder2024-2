fn main() {
    let n = 11;
    let mut tests = Vec::with_capacity(1 << n);
    for s in 0u32..1 << n {
        tests.push(s);
    }
    println!("{}", tests.len());
    for &si in &tests {
        println!("{}", n);
        println!("{:01$b}", si, n);
    }
}

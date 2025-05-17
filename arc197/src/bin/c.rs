use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut is_prime = vec![true; 3_000_000];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..is_prime.len() {
        if is_prime[i] {
            let mut j = i * 2;
            while j < is_prime.len() {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes = is_prime.iter().enumerate().filter(|&(_, &f)| f).map(|(i, _)| i).collect::<Vec<_>>();
    println!("{}", primes.len());
}

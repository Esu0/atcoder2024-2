for ((i = 0; i < 300; i++)) {
    python test.py | cargo run --bin typical90-040 --release -q
}
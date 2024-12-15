for ((i = 0; i < 500; i++)) {
    python gen_test.py | cargo run --bin dp-z --release -q
}
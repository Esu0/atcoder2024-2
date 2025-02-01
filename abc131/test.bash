M=30
for ((i=2; i < M; i++)) {
    for ((j = 0; j < i * (i - 1); j++)) {
        echo "$i $j" | cargo run --bin abc131-e --release
    }
}

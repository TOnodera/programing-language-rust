fn main() {
    let mut count = 0;
    let sum = loop {
        if (10 <= count) {
            break count;
        }
        count += 1;
    };

    println!("{}", sum);
}

mod bid;

fn main() {
    for n in 0..=255 {
        println!("{} {}", n, bid::binary_digit(n));
    }
}

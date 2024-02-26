use bid::binary_digit;

fn main() {
    for n in 0..=255 {
        println!("{} {}", n, binary_digit(n));
    }
}

static LANGUAGE: &'static str = "Rust";
const TRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > TRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The treshold is {}", TRESHOLD);
    println!("{} is {}", n, if is_big(n) {
        "big"
    } else {
        "small"
    });

    // TRESHOLD = 5;
}

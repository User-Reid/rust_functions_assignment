fn apply_to_jobs(x: i32, title: &str) {
    println!("I'm applying to {x} {title} jobs.")
}

fn is_even(x: i32) {
    let even: bool = x % 2 == 0;
    println!("{x} is {even}")
}

fn alphabets(text: &str) {
    let x: bool = text.contains('a') || text.contains('A');
    let y: bool = text.contains('z') || text.contains('Z');
    
    let container: (bool, bool) = (x, y);
    print!("{container:#?}")


    
}

fn main() {
    apply_to_jobs(32, "Rust Developer");

    is_even(32);

    alphabets("ZebrA");
}

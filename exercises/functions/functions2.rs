// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    call_me(3);
}

// interesting functions can be placed after another
fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

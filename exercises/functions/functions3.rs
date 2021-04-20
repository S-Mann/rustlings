fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    // !: Rust doesn't allow default arguments for functions
    // https://www.reddit.com/r/rust/comments/fi6nov/why_does_rust_not_support_default_arguments/
    // https://stackoverflow.com/questions/24047686/default-function-arguments-in-rust
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

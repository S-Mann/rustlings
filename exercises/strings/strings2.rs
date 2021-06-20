fn main() {
    let word = String::from("green");
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

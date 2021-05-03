fn calculate_apple_price(order: i32) -> i32 {
    if order > 40 {
        order
    } else {
        order * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}

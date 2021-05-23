#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    let arr_len = a.len();

    let nice_slice = &a[1..arr_len - 1];

    assert_eq!([2, 3, 4], nice_slice)
}

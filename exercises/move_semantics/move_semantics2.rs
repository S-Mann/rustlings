/*
?: There are 3 ways to solve this
1. Call clone() on vec0 in main(), which will do a copy in heap.
2. Pass vec0 to fill_vec() and call clone() there.
3. Make vec0 mutable pass mutable reference to fill_vec() and completely remove vec1.
*/

fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}

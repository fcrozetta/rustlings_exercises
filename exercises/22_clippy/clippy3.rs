// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

use std::mem::swap;

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
#[allow(clippy::let_unit_value)]
fn main() {
    // !This is wrong, but it works
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("{:?}", my_option);
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // !This is wrong
    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a:i32 = 45;
    let mut value_b:i32 = 66;
    // Let's swap these two!
    swap(&mut value_a, &mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}

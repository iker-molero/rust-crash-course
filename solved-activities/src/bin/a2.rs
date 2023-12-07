// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add_to_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(number_to_display: i32) {
    println!("{:?}", number_to_display);
}

fn main() {
    let added_number: i32 = add_to_numbers(1, 4);
    display_result(added_number);
}

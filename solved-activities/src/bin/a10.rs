// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(bool_value: bool) {
    match bool_value {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}

fn main() {
    let number_value: i32 = 69;
    let bool_value: bool = if number_value > 100 { true } else { false };
    print_message(bool_value);
}

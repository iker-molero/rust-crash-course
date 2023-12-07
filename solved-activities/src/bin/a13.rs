// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector
fn main() {
    let new_vector: Vec<i32> = vec![10, 20, 30, 40];
    for &number in &new_vector {
        if number == 30 {
            println!("thirty");
        } else {
            println!("{:?}", number);
        }
    }
    println!("Vector length: {:?}", &new_vector.len());
}

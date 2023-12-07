// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_name(name: &str) {
    println!("Name is: {:?}", name);
}

fn display_surname(surname: &str) {
    println!("Surname is: {:?}", surname);
}

fn main() {
    let name: &str = "Iker";
    let surname: &str = "Molero";

    display_name(name);
    display_surname(surname);
}

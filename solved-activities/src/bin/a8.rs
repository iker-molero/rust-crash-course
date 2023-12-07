// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Lime,
    Orange,
    Strawberry,
}

struct Drink {
    flavor: Flavor,
    fl_oz: f64,
}

fn print_drink_values(my_drink: Drink) {
    match my_drink.flavor {
        Flavor::Lime => println!("Flavor: Lime"),
        Flavor::Orange => println!("Flavor: Orange"),
        Flavor::Strawberry => println!("Flavor: Strawberry"),
    }
    println!("Fluid Ounces: {:?}", my_drink.fl_oz);
}

fn main() {
    let my_drink: Drink = Drink {
        flavor: Flavor::Strawberry,
        fl_oz: 3.7,
    };
    print_drink_values(my_drink);
}

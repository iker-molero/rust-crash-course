// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

fn main() {
    let people_vector = vec![
        Person {
            age: 7,
            name: "Jack".to_owned(),
            fav_color: "Red".to_owned(),
        },
        Person {
            age: 12,
            name: "John".to_owned(),
            fav_color: "Green".to_owned(),
        },
        Person {
            age: 9,
            name: "Jake".to_owned(),
            fav_color: "Blue".to_owned(),
        },
    ];
    for person in people_vector {
        if person.age <= 10 {
            println!("Name: {:?}, Fav Color: {:?}", person.name, person.fav_color);
        }
    }
}

// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn is_valid(&self) -> Result<&Adult, String> {
        if self.age >= 21 {
            return Ok(self);
        }
        Err("This person is not an adult".to_owned())
    }
}

fn main() {
    let person1 = Adult {
        name: "John".to_owned(),
        age: 19,
    };
    let person2 = Adult {
        name: "Jack".to_owned(),
        age: 22,
    };

    match person1.is_valid() {
        Ok(adult) => println!(
            "{:?} is an adult since they have {:?} years",
            adult.name, adult.age
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    match person2.is_valid() {
        Ok(adult) => println!(
            "{:?} is an adult since they have {:?} years",
            adult.name, adult.age
        ),
        Err(e) => println!("Error: {:?}", e),
    }
}

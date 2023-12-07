// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let students_vector = vec![
        Student {
            name: "John".to_owned(),
            locker_assignment: Some(3732),
        },
        Student {
            name: "Jack".to_owned(),
            locker_assignment: None,
        },
    ];

    for student in students_vector {
        println!("Name: {:?}", student.name);
        match student.locker_assignment {
            Some(id) => println!("Locker: {:?}", id),
            None => println!("Locker: -"),
        };
    }
}

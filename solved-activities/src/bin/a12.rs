// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Green,
    Blue,
}

struct ShippingBox {
    dimensions: (i32, i32, i32),
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn print_characteristics(&self) {
        println!("{:?}", self.dimensions);
        println!("{:?}", self.weight);
        match self.color {
            BoxColor::Red => println!("Red"),
            BoxColor::Green => println!("Green"),
            BoxColor::Blue => println!("Blue"),
        }
    }

    fn big_green() -> Self {
        Self {
            dimensions: (10, 5, 7),
            weight: 20.3,
            color: BoxColor::Green,
        }
    }
}

fn main() {
    let my_box: ShippingBox = ShippingBox {
        dimensions: (2, 4, 1),
        weight: 4.5,
        color: BoxColor::Blue,
    };
    my_box.print_characteristics();
    let other_box: ShippingBox = ShippingBox::big_green();
    other_box.print_characteristics();
}

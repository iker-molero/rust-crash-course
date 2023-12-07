// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket {
    Vip(f64, String),
    Backstage(f64, String),
    Standard(f64),
}

fn main() {
    let tickets_vector: Vec<Ticket> = vec![
        Ticket::Vip(49.99, "John".to_owned()),
        Ticket::Backstage(24.99, "Jack".to_owned()),
        Ticket::Standard(14.99),
    ];

    for ticket in tickets_vector {
        match ticket {
            Ticket::Vip(price, name) => println!("VIP ticket for: {:?} costs: {:?}", name, price),
            Ticket::Backstage(price, name) => {
                println!("Backstage ticket for: {:?} costs: {:?}", name, price)
            }
            Ticket::Standard(price) => println!("Standar ticket costs: {:?}", price),
        }
    }
}

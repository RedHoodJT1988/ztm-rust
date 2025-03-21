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
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Tommy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Kimberly".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => {
                println!("Backstage ticket for {} costs ${}", name, price);
            }
            Ticket::Standard(price) => {
                println!("Standard ticket costs ${}", price);
            }
            Ticket::Vip(price, name) => {
                println!("Vip ticket for {} costs ${}", name, price);
            }
        }
    }
}


// use rand::Rng;
use std::io;

fn main() {
    /* 1/8/21 
    let mut rand = rand::thread_rng().gen_range(1,101);
    println!("rand = {}", rand);
    rand = rand::thread_rng().gen_range(1,101);
    println!("rand = {}", rand);
    */

    /* 1/9/21 */
    let mut input = String::new();
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("Enter a number 1-12: ");
    io::stdin()
        .read_line(&mut input)
        .expect("error reading from stdin");

    // take input number and output corresponding month
    let parse_input: usize = input.trim().parse().unwrap(); //need error handling
    if parse_input > 0 && parse_input < 13 {
        println!("That corresponds to the month of {}", months[parse_input - 1]); // -1 since index starts at 0
    } else {
        println!("not in month range");
    }
}
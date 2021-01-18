// use rand::Rng;
use std::io;

fn main() {
    /* 1/8/21 
    let mut rand = rand::thread_rng().gen_range(1,101);
    println!("rand = {}", rand);
    rand = rand::thread_rng().gen_range(1,101);
    println!("rand = {}", rand);
    */

    /* 1/9/21 - 1/11/21 */
    let mut input = String::new();
    let mut input_month = String::new();

    // take input number and output corresponding month
    println!("Enter a number 1-12: ");
    io::stdin()
        .read_line(&mut input)
        .expect("error reading from stdin");
    let input: usize = input.trim().parse().unwrap_or_default();
    to_month(input, &mut input_month);
    println!("{}", input_month);
}

/*  params: input (index of month), month (empty borrowed mutable String,
    used for return value) 
    1/17/21 */
fn to_month(input: usize, month: &mut String){
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    if input > 0 && input < 13 {
        month.push_str(months[input - 1]); // subtract 1 since index starts at 0
    } else {
        month.push_str("n/a");
    }
}
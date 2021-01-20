// use rand::Rng;
use std::io;
use std::convert::TryFrom;

struct Birthday {
    date_of_birth: String,
    day: u32,
    month_i: u32,
    month: String,
    year: u32
}

fn main() {  
    let mut input = Birthday {
        date_of_birth: String::new(),
        day: 0u32,
        month_i: 0u32,
        month: String::new(),
        year: 0u32
    };

    // take input number and output corresponding month
    println!("Enter your birthday (dd/mm/yyyy): ");
    io::stdin()
        .read_line(&mut input.date_of_birth)
        .expect("Error reading from stdin");

    input = get_birthday(input);
    if input.month == "n/a" {
        println!("Check your inputs");
    } else {
        println!("You were born on {} {}, {}", input.day, input.month, input.year);
    }
}
/*  params: input (index of month), month (borrowed mutable String,
    used for return value) */
fn to_month(index: usize, month: &mut String){
    month.clear();
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    if index > 0 && index < 13 {
        month.push_str(months[index - 1]); // subtract 1 since index starts at 0
    } else {
        month.push_str("n/a");
    }
}
/*  params: input, of type struct Birthday
    returns: variable of type struct Birthday */
fn get_birthday(mut input: Birthday) -> Birthday{
    // search for separators in date and take the data from between them
    let input_b = input.date_of_birth.as_bytes();
    let mut found = 0u32; // number of found separators
    let mut f = 0usize; // placeholder for index
    for (i, &item) in input_b.iter().enumerate() {
        if (item == b'/' || item == b'-' || item == b'\\') && found == 0 {
            input.day = input.date_of_birth[..i].trim().parse().unwrap_or_default();
            f = i + 1;
            found += 1;
        } else if (item == b'/' || item == b'-' || item == b'\\') && found == 1 {
            input.month_i = input.date_of_birth[f..i].trim().parse().unwrap_or_default();
            to_month(usize::try_from(input.month_i).unwrap_or_default(), &mut input.month);
            f = i + 1;
            found += 1;
        } else if found == 2 {
            input.year = input.date_of_birth[f..].trim().parse().unwrap_or_default();
            break;
        }
    }
    return input;
}
use rand::Rng;

fn main() {
    let mut rand = rand::thread_rng().gen_range(1,101);
    println!("rand = {}", rand);
    rand = rand::thread_rng().gen_range(1,101);
    println!("rand = {}", rand);
}

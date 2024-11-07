mod evidence;
mod courtcase;

fn main() {
    let cases = courtcase::load_cases();
    println!("{:?}", cases);
}

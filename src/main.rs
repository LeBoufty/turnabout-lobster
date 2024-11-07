mod evidence;

fn main() {
    let record = evidence::load_evidence(String::from("cases/example/evidence.toml"));
    println!("{:?}", record);
}

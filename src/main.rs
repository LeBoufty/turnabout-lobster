mod evidence;
mod courtcase;
mod dialogue;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cases = courtcase::load_cases();
    let evidence = evidence::load_evidence(&cases[0]);
    let selected_evidence = evidence::select_evidence(&evidence);
    println!("Selected evidence: {}", selected_evidence.get_fullname());
    let testimonies = dialogue::get_all_testimonies(&cases[0]);
    for testimony in testimonies {
        testimony.play();
    }
    Ok(())
}

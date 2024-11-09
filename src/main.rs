mod evidence;
mod courtcase;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cases = courtcase::load_cases();
    let evidence = evidence::load_evidence(&cases[0]);
    let selected_evidence = evidence::select_evidence(&evidence)?;
    println!("Selected evidence: {}", selected_evidence.get_fullname());
    Ok(())
}

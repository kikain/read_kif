use read_kif::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kif = Kif::new(r".\data\kif1.kif2")?;
    println!("{:#?}", kif.kif.iter().take(10).collect::<Vec<_>>());
    Ok(())
}

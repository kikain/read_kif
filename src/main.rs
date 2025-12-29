use read_kif_case::*;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let readed = Kif::new(r".\data\kif1.kif2")?;
    println!("{:#?}",readed.kif.iter().take(10).collect::<Vec<_>>());
    Ok(())
}
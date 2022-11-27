use csv::Writer;
use std::error::Error;

#[allow(dead_code)]
/// Write vector to .csv
pub fn write_vec_csv(g: &[i8], path: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(g.iter().map(|x| x.to_string()))?;
    wtr.flush()?;
    Ok(())
}

use std::fs::File;

use serde::Serialize;

pub fn write_json<T: Serialize>(filename: String, data: T) -> anyhow::Result<()> {
    let file = File::create(filename)?;
    // TODO: Optional pretty output
    serde_json::to_writer_pretty(file, &data)?;
    Ok(())
}

#[allow(dead_code)]
pub fn write_csv<T: Serialize>(filename: String, data: &[T]) -> anyhow::Result<()> {
    let file = File::create(filename)?;
    let mut csv_writer = csv::Writer::from_writer(file);

    for record in data {
        let r = csv_writer.serialize(record);
        if let Err(e) = r {
            eprintln!("{}", e);
        }
    }

    csv_writer.flush()?;
    Ok(())
}

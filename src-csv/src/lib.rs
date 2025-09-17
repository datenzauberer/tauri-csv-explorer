use polars::prelude::*;
use std::env;

pub fn show_sniffer_metadata(path: &str) {
    // sniff the path provided by the first argument
    match qsv_sniffer::Sniffer::new().sniff_path(path) {
        Ok(metadata) => {
            println!("Metadata for: {}", path);
            println!("{}", metadata);
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
    }
}

pub fn read_csv_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = read_csv_file_as_json_as_string(path)?;
    println!("csv as json:{}", json_string);
    Ok(())
}

pub fn read_csv_file_as_json_as_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from_utf8(read_csv_file_as_json_as_u8(path)?)?)
}

fn read_csv_file_as_json_as_u8(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // --- Sniff the file to get its properties ---
    let metadata = qsv_sniffer::Sniffer::new().sniff_path(path)?;
    // --- Read the file using the sniffed properties ---
    let dialect = metadata.dialect;

    let mut df = LazyCsvReader::new(PlPath::new(path))
        .with_skip_rows(dialect.header.num_preamble_rows)
        .with_has_header(dialect.header.has_header_row)
        .with_separator(dialect.delimiter)
        .finish()?
        .collect()?;

    let mut buffer = Vec::new();
    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df)?;

    Ok(buffer)
}


use csv_explorer::read_csv_file_as_json_as_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "../test-data/test-file-short.csv";
    show_sniffer_metadata(path);
    read_csv_file(path)?;
    Ok(())
}

fn show_sniffer_metadata(path: &str) {
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

fn read_csv_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = read_csv_file_as_json_as_string(path)?;
    println!("csv as json:{}", json_string);
    Ok(())
}

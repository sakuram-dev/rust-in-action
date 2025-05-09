fn main() {
    let penguin_data = "\
        common name, length (cm)
        Lttle Penguin, 33
        Yellow-eyed Penguins, 65
        Fiordland Penguin, 60
        Invalid, data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        let record = record.trim();
        
        if i == 0 || record.len() == 0 {
            continue;
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}: {} cm", name, length);
        }
    }
}

fn main() {
    // defining a sample csv
    let data = "\
    name,age
    ravi,20
    adithya,20
    jhraddha,20
    hehe,20
    Invalid,data
    ";

    let records = data.lines();

    for (i, record) in records.enumerate(){
        if i == 0 || record.trim().len() == 0{
            continue;
        }
        let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
        // a debug to show what might be happening
        if cfg!(debug_assertions){
            eprintln!("debug: {:?} -> {:?}",
                    record, fields);
        }
        // the debug output is supressed if we 
        // cargo run using --release flag
        // ie, debug is not considered in release versions
        let name = fields[0];
        if let Ok(age) = fields[1].parse::<i32>(){
            println!("{} is {} years old", name, age);
        }
    }
}

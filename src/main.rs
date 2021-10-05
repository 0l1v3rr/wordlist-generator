use std::io::{stdin,stdout,Write};
use std::fs::File;
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let mut text = String::new();
    print!("Enter the words separated by semicolons: ");
    let _ = stdout().flush();
    stdin().read_line(&mut text).expect("Please enter a correct string.");

    if let Some('\n') = text.chars().next_back() {
        text.pop();
    }
    if let Some('\r') = text.chars().next_back() {
        text.pop();
    }

    let mut txt_name = String::new();
    print!("Enter the name of the wordlist: ");
    let _ = stdout().flush();
    stdin().read_line(&mut txt_name).expect("Please enter a correct string!");
    
    if let Some('\n') = txt_name.chars().next_back() {
        txt_name.pop();
    }
    if let Some('\r') = txt_name.chars().next_back() {
        txt_name.pop();
    }

    let split = text.split(";");
    let splitted: Vec<&str> = split.collect();

    let mut words = Vec::new();
    let mut nums = Vec::new();

    for s in splitted {
        if s.parse::<f64>().is_ok() {
            nums.push(s);
        } else {
            words.push(s.to_lowercase());
            words.push(s.to_uppercase());
            words.push(uppercase_first_letter(s));
        }
    }

    println!("Generating words...");
    let mut result_count = 0;
    let mut file = File::create(txt_name + ".txt")?;

    for i in &words {
        for j in &nums {
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("-").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("_").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("%").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("/").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("-").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("_").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("%").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("/").as_bytes())?;
            file.write("\n".as_bytes())?;
            result_count += 10;
        }
    }

    for i in &words{
        for j in &words {
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("-").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("_").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("%").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned().to_owned()].join("/").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i.to_owned()].join("").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i.to_owned()].join("-").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i.to_owned()].join("_").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i.to_owned()].join("%").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i.to_owned()].join("/").as_bytes())?;
            file.write("\n".as_bytes())?;
            result_count += 10;
        }
    }

    for i in &nums{
        for j in &nums {
            file.write_all([i.to_owned(), j.to_owned()].join("").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned()].join("-").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned()].join("_").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned()].join("%").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([i.to_owned(), j.to_owned()].join("/").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("-").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("_").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("%").as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write_all([j.to_owned(), i].join("/").as_bytes())?;
            file.write("\n".as_bytes())?;
            result_count += 10;
        }
    }

    println!("Done! Generated words: {} words", result_count);

    Ok(())
}

fn uppercase_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(chars).collect(),
    }
}
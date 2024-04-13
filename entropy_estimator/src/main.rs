use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    // Opens a file called example.txt
    let file_path = "example.txt";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Failed to open file '{}'", file_path);
            return;
        }
    };

    // Reads the file
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    if let Err(_) = reader.read_to_end(&mut buffer) {
        eprintln!("Error: Failed to read file '{}'", file_path);
        return;
    }

    // Compute frequency of each byte 
    let mut frequencies = HashMap::new();
    for byte in &buffer {
        *frequencies.entry(*byte).or_insert(0) += 1;
    }

    // Calculate probability of each byte value
    let total_bytes = buffer.len() as f64;
    let probabilities: HashMap<u8, f64> = frequencies
        .into_iter()
        .map(|(byte, frequency)| (byte, frequency as f64 / total_bytes))
        .collect();

    // Calculate Shannon's entropy -> -p*log(p)
    let entropy = probabilities
        .values()
        .map(|&prob| -prob * prob.log2())
        .sum::<f64>();

    println!("Shannon's Entropy: {:.6}", entropy);
}


use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::path::Path;
use pyo3::pyclass;

//to implement 

#[pyclass]
pub struct FastaSequence{

    metadata: String,
    sequence: String, 

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_dna_only(path: &String) -> Vec<String> {

    let mut sequences= Vec::<String>::new();
    
        // get lines
        if let Ok(lines) = read_lines(path) {

            let mut seq = String::from("");

            //for each line in text
            for line in lines {
                
                //define dna as line 
                if let Ok(dna) = line {

                    //check if line is metadata (begining of new sequence)
                    if &dna[0..1] != ">" {
                        seq.push_str(&dna);
                    }
                    else {
                        sequences.push(seq);
                        seq = String::from("");
                    }
                }
            }
            
        }
    sequences

}


pub fn get_metadata(path: &String) -> Vec<String> {

    let mut metadata= Vec::<String>::new();

    if let Ok(lines) = read_lines(path) {

        for line in lines {
            
            if let Ok(data) = line {

                if &data[0..1] == ">" {
                    metadata.push(data);
                }
                
            }
        }
        
    }

    metadata
}


pub fn write_to_file(path: &String, data:&Vec<String>, metadata: &Vec<String>) {

    if !Path::new(path).exists() {
        File::create(path).expect("Error encountered while creating file!");
    }

    let mut f= OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();


    for i in 0..data.len() {

        if let Err(e)= writeln!{f, "{}\n{}", metadata[i], data[i]} {
            eprintln!("Couldn't write to file: {}", e);
        }

        
    }
} 


pub fn get_sequences(path: &String) -> Vec<FastaSequence> {

    let mut sequences = Vec::<FastaSequence>::new();

    if let Ok(lines) = read_lines(path) {

        let mut current_seq = String::from("");
        let mut current_metadata= String::from("");
        
        for line in lines{

            if let Ok(data) = line {

                if &data[0..1] == ">" {

                    if current_metadata.len() > 0 {

                        sequences.push(FastaSequence { metadata: current_metadata.clone(), sequence: current_seq.clone() });
                        let mut current_metadata= String::from("");
                        let mut current_seq = String::from("");
                    }
                    current_metadata.push_str(&data);
                
                }
            
                else{
                    current_seq.push_str(&data)
                }
            }
        }

    }
    return sequences
}

use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct FastaSequence{

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

//to test 
pub fn write_to_file(path: &String, data:&Vec<String>, metadata: &Vec<String>) {

    for i in 0..data.len() {

        fs::write(path, &metadata[i]);
        fs::write(path, "\n");
        fs::write(path, &data[i]);
        fs::write(path, "\n");

    }
}
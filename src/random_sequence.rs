
use rand::seq::IteratorRandom;
use pyo3::prelude::*;

fn random_seq(lenght: i64, seq_type: &str) -> String{

    
    let mut rand_seq= String::from("");
    let nt_dna= "atgc";
    let nt_arn= "aucg";
    let aa= "GAVCPLIMWFSTYNQKRHDE";
    let mut rng = rand::thread_rng();
    

    match seq_type.to_lowercase().as_str() {

        "dna"  => for _i in 0..lenght { rand_seq.push(
            nt_dna.chars().choose(&mut rng).unwrap());},

        "rna" => for _i in 0..lenght { rand_seq.push(
            nt_arn.chars().choose(&mut rng).unwrap());},

        "aa" => for _i in 0..lenght{ rand_seq.push(
            aa.chars().choose(&mut rng).unwrap());},
        
        _ => panic!("Choose a valid type of sequence!(dna, rna or aa for amino acid sequence)")
    }

    return rand_seq

}

#[pyfunction]
pub fn rand_sequences(length: i64, seq_type: String) -> String {

    let return_seq= random_seq(length, &seq_type);
    return return_seq
}
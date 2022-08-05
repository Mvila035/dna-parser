
use rand::seq::IteratorRandom;

pub fn random_seq(lenght: i64, seq_type: &str) -> String{

    
    let mut rand_seq= String::from("");
    let nt_dna= "atgc";
    let nt_arn= "aucg";
    let aa= "GAVCPLIMWFSTYNQKRHDE";
    let mut rng = rand::thread_rng();


    if seq_type== "dna"{

        for _i in 0..lenght{

            rand_seq.push(
                nt_dna.chars().choose(&mut rng).unwrap()
            );

        }
        
    }

    else if seq_type== "rna"{

        for _i in 0..lenght{

            rand_seq.push(
                nt_arn.chars().choose(&mut rng).unwrap()
            );
        }
    }

    else if seq_type== "aa" {

        for _i in 0..lenght{
            
            rand_seq.push(
            aa.chars().choose(&mut rng).unwrap()
            );
        }
    }

    return rand_seq

}

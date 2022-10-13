
pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod fasta_files;
pub mod tfidf;
pub mod ordinal;




use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyModule;
use pyo3::PyResult;

use onehot::*;
use kmers::*;
use random_sequence::*;
use tfidf::*;
use ordinal::*;




#[pymodule]
fn dna_parser(_py: Python<'_>, m: &PyModule)-> PyResult<()> {

    m.add_function(wrap_pyfunction!(onehot_encoding,m)?)?;
    m.add_function(wrap_pyfunction!(make_kmers,m)?)?;
    m.add_function(wrap_pyfunction!(rand_sequences,m)?)?;
    m.add_function(wrap_pyfunction!(tfidf_encoding,m)?)?;
    m.add_function(wrap_pyfunction!(ordinal_encoding,m)?)?;
    m.add_function(wrap_pyfunction!(batch_ordinal_encoding,m)?)?;
    m.add_function(wrap_pyfunction!(ordinal_from_fasta,m)?)?;


    Ok(())
}



#[cfg(test)]
mod tests {

    
    use crate::fasta_files::get_dna_only;

    #[test]
    fn fasta(){

        let file= String::from("/Users/matthieuvilain/Desktop/Master/research_project/database/alignements/AlphaDelta_TexasMinnesota_AprMayAug_Mafft_NoIndel.fasta");
        let corpus= get_dna_only( &file);
        println!("{}",corpus[1]);


    }

}


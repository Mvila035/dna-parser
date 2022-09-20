pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod fasta_files;
pub mod tfidf;



use fasta_files::FastaSequence;
use numpy::{IntoPyArray, PyArray1, PyArray2};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyModule;
use pyo3::PyResult;

use onehot::*;
use kmers::make_kmers;
use random_sequence::random_seq;
use fasta_files::get_sequences;
use tfidf::tf_idf;



#[pyfunction]
fn tfidf_encoding<'pyt>(py: Python <'pyt>, corpus: Vec<String>) -> &'pyt PyArray2<f64> {

    let array= tf_idf(corpus);
    let py_array= array.into_pyarray(py);

    py_array
}

#[pyfunction]
fn GetSequences(path: String)-> Vec<FastaSequence> {

    let sequences= get_sequences(&path);
    return sequences
}


#[pyfunction]
fn onehot_encoding<'pyt>(py:  Python <'pyt>, seq: String) ->  &'pyt PyArray2<i8>{


    let  matrix= one_hot(&seq);
    let pyarray= matrix.into_pyarray(py);

    return pyarray
    
}


#[pyfunction]
fn ordinal_encoding<'pyt>(py:  Python <'pyt>, seq: String) ->  &'pyt PyArray1<f32>{


    let  matrix= ordinal(&seq);
    let pyarray= matrix.into_pyarray(py);

    return pyarray
    
}




#[pyfunction]
fn MakeKmers(seq: String, k: i64) -> String {

    let return_seq= make_kmers(k,&seq);

    return return_seq
}

#[pyfunction]
fn rand_sequences(length: i64, seq_type: String) -> String {

    let return_seq= random_seq(length, &seq_type);
    return return_seq
}

#[pymodule]
fn dna_parser(_py: Python<'_>, m: &PyModule)-> PyResult<()> {

    m.add_function(wrap_pyfunction!(onehot_encoding,m)?)?;
    m.add_function(wrap_pyfunction!(MakeKmers,m)?)?;
    m.add_function(wrap_pyfunction!(rand_sequences,m)?)?;
    m.add_function(wrap_pyfunction!(GetSequences,m)?)?;
    m.add_function(wrap_pyfunction!(tfidf_encoding,m)?)?;
    m.add_function(wrap_pyfunction!(ordinal_encoding,m)?)?;


    Ok(())
}



#[cfg(test)]
mod tests {

    use crate::tfidf::tf_idf;

    #[test]
    fn tfidf_test(){

        
        let corpus= vec![String::from("ACGT ACGT ATTT AGGG"), String::from("ACGT ACCT ACGT ACCC"), String::from("ACGT ACCC AGTC ACGT")];
        let array= tf_idf(corpus);
        println!("{}",array);


    }

}


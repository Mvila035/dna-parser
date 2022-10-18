use numpy::ndarray::{Dim, OwnedRepr, ArrayBase, Array,Array1};
use numpy::{IntoPyArray, PyArray1};
use std::sync::{Arc, Mutex};
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::thread;
use crate::fasta_files::get_dna_only;
use num_cpus;



pub fn ordinal(sequence: &str)-> Array1<f32>{

    let ncols= sequence.len();
    let mut vec= Array1::<f32>::zeros(ncols);
   
    for elements in sequence.chars().enumerate() {

        let (index, charac)= elements;
        match charac {

            'A' => vec[index]= 0.25,
            'C' => vec[index]= 0.50,
            'G' => vec[index]= 0.75,
            'T' => vec[index]= 1.0,
            'U' => vec[index]= 1.0,
            'a' => vec[index]= 0.25,
            'c' => vec[index]= 0.50,
            'g' => vec[index]= 0.75,
            't' => vec[index]= 1.0,
            'u' => vec[index]= 1.0,
            _ => vec[index]= 0.0


        }       

    }
    let matrix= Array::from(vec);

    matrix
}




#[allow(unused_must_use)]
#[pyfunction]
pub fn batch_ordinal_encoding<'pyt>(py:  Python <'pyt>, sequences: Vec<&str>, n_jobs: i16 ) ->  &'pyt PyList{

    
    let py_list= PyList::empty(py);

    if n_jobs != 1 {

        let results= multithreads(sequences, n_jobs);

        for elements in results {

            for seq in elements.1 {

                py_list.append(seq.into_pyarray(py));
            }
        }

        return py_list
    }

    else {

        for seq in sequences{

            let encoding= ordinal(seq);
            
            py_list.append(encoding.into_pyarray(py));
            
        }

        return py_list

    }    
}


#[allow(unused_must_use)]
#[pyfunction]
pub fn ordinal_from_fasta<'pyt>(py: Python  <'pyt>,path: String) -> &'pyt PyList{

    let sequences= get_dna_only(&path);
    let list = PyList::empty(py);

    for seq in sequences{

        let encoding= ordinal(&seq);
        
        list.append(encoding.into_pyarray(py));
        
    }

    
    list
}


#[pyfunction]
pub fn ordinal_encoding<'pyt>(py:  Python <'pyt>, seq: &str) ->  &'pyt PyArray1<f32>{


    let  matrix= ordinal(seq);
    let pyarray= matrix.into_pyarray(py);

    return pyarray
    
}




fn multithreads(sequences: Vec<&str>, n_jobs: i16)-> Vec<(usize, Vec<ArrayBase<OwnedRepr<f32>, Dim<[usize; 1]>>>)> {

    let threads;

    if n_jobs == 0{

        threads= num_cpus::get_physical() as i16;
    }

    else {

        threads= n_jobs;
    }

    let seq_len= sequences.len();
    let slice_len= seq_len/ threads as usize;

    let results= Arc::new(Mutex::new(Vec::new()));

// ####################################### begining or threads #####################################
    thread::scope(|s|{

        let results= &results;
        for (index,chunk) in sequences.chunks(slice_len).enumerate(){

            s.spawn( move || {

                
                let mut vec_to_push= Vec::new();

                for seq in chunk.into_iter(){

                    let encoding= ordinal(seq);
                    
                    vec_to_push.push(encoding);
                    
                }

                results.lock().unwrap().push((index, vec_to_push));

            });

        }

    });


// ####################################### end or threads #####################################


    let mutex_vec= Arc::try_unwrap(results).unwrap();

    let mut result_vec= mutex_vec.into_inner().unwrap();

    result_vec.sort_by_key(|k| k.0);

    result_vec

    

}
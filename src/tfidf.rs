
use numpy::ndarray::ArrayView;
use numpy::ndarray::Array2;
use std::collections::HashMap;
use std::collections::HashSet;


fn map_vocabulary(corpus: &Vec<String>) -> (HashMap<&str,f64>, Vec<&str>) {

    let mut map= HashMap::new();
    let mut word_order= Vec::new();
    let mut words_set= HashSet::new();

    for seq in corpus.iter() {

        let mut current_words= HashSet::new();

        for word in seq.split_whitespace(){
            
            if !words_set.contains(word) {
                word_order.push(word);
                words_set.insert(word);
            }
            
            if !current_words.contains(word){
                map.entry(word).and_modify(|counter| *counter += 1.0).or_insert(1.0);
                current_words.insert(word);
            }
        }

    }

    (map,word_order)
}

fn word_counts(sequence: &String) -> HashMap<&str,f64> {

    let mut counts= HashMap::new();

    for word in sequence.split_whitespace(){

        counts.entry(word).and_modify(|counter| *counter += 1.0).or_insert(1.0);
    }

    counts
}

fn compute_tfidf(length:f64, counts: HashMap<&str, f64>, map: &(HashMap<&str,f64>, Vec<&str>)) -> Vec<f64>{

    let mut tfidf_vec= Vec::new();

    for word in map.1.iter(){

        let tf= counts.get(word).unwrap()/length;
        let idf: f64=  ( map.0.len() as f64 / ( map.0.get(word).unwrap()+1.0 )).ln();
        let tfidf= tf*idf;

        tfidf_vec.push(tfidf);

    }

    tfidf_vec
}

pub fn tf_idf(corpus: Vec<String>) -> Array2<f64> {


    let word_map= map_vocabulary(&corpus);
    let nrows= corpus.len();
    let ncols= word_map.0.len();

    let mut matrix =Array2::<f64>::zeros((nrows,ncols));

    for seq in corpus.iter() {

        let seq_len= seq.split_whitespace().count() as f64;
        let counts= word_counts(seq);

        let tfidf_vec= compute_tfidf(seq_len, counts, &word_map);
        matrix.push_row(ArrayView::from(&tfidf_vec)).unwrap();
        
    }

    
    matrix
}
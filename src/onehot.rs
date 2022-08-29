use numpy::ndarray::{Array, ArrayView, array};
use numpy::ndarray::ArrayBase;
use numpy::ndarray::Array2;

pub fn one_hot(sequence: &str)-> Array2<i8> {

    let seq= sequence.to_lowercase();
    let mut matrix= Array2::<i8>::zeros((0,4));

    //define values for one hot encoding
    for charac in seq.chars() {

        match charac {

            'a' => matrix.push_row(ArrayView::from(&[ 1,  0,  0,  0])).unwrap(),
            'c' => matrix.push_row(ArrayView::from(&[ 0,  1,  0,  0])).unwrap(),
            'g' => matrix.push_row(ArrayView::from(&[ 0,  0,  1,  0])).unwrap(),
            't' => matrix.push_row(ArrayView::from(&[ 0,  0,  0,  1])).unwrap(),
            _ => continue
        }
    }
    return matrix
}


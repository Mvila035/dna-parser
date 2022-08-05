pub fn one_hot(sequence: &str)-> Vec<Vec<i32>> {

    let seq= sequence.to_lowercase();
    let mut vec= Vec::new();

    //define values for one hot encoding

    for charac in seq.chars() {

        if charac== 'a' {

            vec.push(vec![1,0,0,0]);
        }

        else if charac== 'c'{

            vec.push(vec![0,1,0,0]);
        }

        else if charac== 'g'{

            vec.push(vec![0,0,1,0]);
        }

        else if charac== 't'{

            vec.push(vec![0,0,0,1]);
        }

    }
    
    return vec
}
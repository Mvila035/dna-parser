
pub fn make_kmers(k: i64, sequence: &str) -> String {

    let mut new_str= String::from("");
    let k_usize= k as usize;
    for (i,c) in sequence.chars().enumerate() {

        new_str.push(c);

        if (i+1)%k_usize == 0 {

            new_str.push_str(" ");
    
        }
        
    }   
    return new_str
}
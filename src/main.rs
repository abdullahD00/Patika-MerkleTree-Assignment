use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use sha2::{Sha256, Digest};


fn main(filename: String) -> String{
    // Read  Data 
    let file = File::open(filename).unwrap();
    let mut line = String::new();
    let mut reader = BufReader::new(file);

    // Vector words 
    let mut text_words = Vec::new();
    // Vector hashes
    let mut text_hashes = Vec::new();


    // Read words and add to vector
    for line in reader.lines(){
        text_words.push(line.unwrap());
    }

    // Hash the words and add to vector
    for i in 1..text_words.len(){
        let line_hash = hash_input(&text_words[i]);
        text_hashes.push((&line_hash).to_string());
        println!("Word is --{}-- hash is {}", text_words[i], text_hashes[i-1])
    }
    
    //2^n 
    if text_hashes.len()%2!= 0 {
        let last_element = text_hashes[text_hashes.len()-1].to_string();
        text_hashes.push(last_element)
    }

    // Find root hash
    let root_hash = create_next_level(text_hashes);
    println!("Root hash : {}", root_hash);
   
        return  root_hash.to_string()

    
}

// Calculate root hash
fn create_next_level(current_level: Vec::<String>) -> String {
    let mut length_of_hash = current_level.len();

    let mut new_vec = current_level.clone();
    println!("second hashes came incoming :{:?}",new_vec);

    let mut hashes = Vec::new();
    
    for i in 0..((length_of_hash/2)){
        for j in (0..(length_of_hash-1)).step_by(2){
            let first_string = &new_vec[j].to_owned();
            let second_string = &new_vec[j+1].to_owned();
            let together  = format!("{}{}", first_string, second_string);
            let line_hash = hash_input(&together);
            hashes.push((&line_hash).to_string());
        }
        if length_of_hash > 1 {
            length_of_hash = length_of_hash/2;        
        }
        new_vec = hashes.clone();
        println!("{} hashes {:?}",i,new_vec);
        if hashes.len() !=1 {
        hashes = Vec::new();
        }
        println!("{:?}",hashes);
    }
   
    hashes[hashes.len()-1].to_string()
}


fn hash_input(input: &str) -> String {
    let mut makehash = Sha256::new();
    makehash.update(input);
    let hash = makehash.finalize();
    let hex = hex::encode(&hash);

    return hex.to_string();
}

// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = main("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = main("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = main("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = main("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = main("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}
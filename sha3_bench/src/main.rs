use sha3::{Digest, Keccak224, Keccak256, Keccak256Full, Keccak384,
        Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;


fn file_to_vec(the_string: &str) -> Vec<String> {
    let mut str_vec = Vec::new();
    let the_file = File::open(the_string).expect("File error");
    let the_reader = BufReader::new(the_file);

    for line in the_reader.lines() {
        let trim_line = line.expect("Line error");
        str_vec.push(trim_line);
    }
    str_vec
}


fn main() {
    let vec_o_strings = file_to_vec("../passed-basic-check.txt");

//******************* Keccak224 ******************\\

    let now1 = Instant::now();
    for a_string1 in &vec_o_strings {
        let mut hash1 = Keccak224::new();
        hash1.update(a_string1);
        hash1.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later1 = now1.elapsed().subsec_nanos();
    println!("{}", later1);

//******************* Keccak256 ******************\\

    let now2 = Instant::now();
    for a_string2 in &vec_o_strings {
        let mut hash2 = Keccak256::new();
        hash2.update(a_string2);
        hash2.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later2 = now2.elapsed().subsec_nanos();
    println!("{}", later2);

//******************* Keccak256Full ******************\\

    let now3 = Instant::now();
    for a_string3 in &vec_o_strings {
        let mut hash3 = Keccak256Full::new();
        hash3.update(a_string3);
        hash3.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later3 = now3.elapsed().subsec_nanos();
    println!("{}", later3);

//******************* Keccak384 ******************\\

    let now4 = Instant::now();
    for a_string4 in &vec_o_strings {
        let mut hash4 = Keccak384::new();
        hash4.update(a_string4);
        hash4.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later4 = now4.elapsed().subsec_nanos();
    println!("{}", later4);

//******************* Keccak512 ******************\\

    let now5 = Instant::now();
    for a_string5 in &vec_o_strings {
        let mut hash5 = Keccak512::new();
        hash5.update(a_string5);
        hash5.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later5 = now5.elapsed().subsec_nanos();
    println!("{}", later5);

//******************* Sha3_224 ******************\\

    let now6 = Instant::now();
    for a_string6 in &vec_o_strings {
        let mut hash6 = Sha3_224::new();
        hash6.update(a_string6);
        hash6.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later6 = now6.elapsed().subsec_nanos();
    println!("{}", later6);

//******************* Sha3_256 ******************\\

    let now7 = Instant::now();
    for a_string7 in &vec_o_strings {
        let mut hash7 = Sha3_256::new();
        hash7.update(a_string7);
        hash7.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later7 = now7.elapsed().subsec_nanos();
    println!("{}", later7);

//******************* Sha3_384 ******************\\

    let now8 = Instant::now();
    for a_string8 in &vec_o_strings {
        let mut hash8 = Sha3_384::new();
        hash8.update(a_string8);
        hash8.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later8 = now8.elapsed().subsec_nanos();
    println!("{}", later8);

//******************* Sha3_512 ******************\\

    let now9 = Instant::now();
    for a_string9 in &vec_o_strings {
        let mut hash9 = Sha3_512::new();
        hash9.update(a_string9);
        hash9.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later9 = now9.elapsed().subsec_nanos();
    println!("{}", later9);

/*
    let mut hash1 = Keccak224::new();
    let mut hash2 = Keccak256::new();
    let mut hash3 = Keccak256Full::new();
    let mut hash4 = Keccak384::new();
    let mut hash5 = Keccak512::new();
    let mut hash6 = Sha3_224::new();
    let mut hash7 = Sha3_256::new();
    let mut hash8 = Sha3_384::new();
    let mut hash9 = Sha3_512::new();

    hash1.update("hello");
    hash2.update("hello");
    hash3.update("hello");
    hash4.update("hello");
    hash5.update("hello");
    hash6.update("hello");
    hash7.update("hello");
    hash8.update("hello");
    hash9.update("hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();
    let res3 = hash3.finalize();
    let res4 = hash4.finalize();
    let res5 = hash5.finalize();
    let res6 = hash6.finalize();
    let res7 = hash7.finalize();
    let res8 = hash8.finalize();
    let res9 = hash9.finalize();

    println!("\nThe Keccak224 hash for \"Hello\" is:");
    println!("{:x}\n", res1);
    println!("The Keccak256 hash for \"Hello\" is:");
    println!("{:x}\n", res2);
    println!("The Keccak256Full hash for \"Hello\" is:");
    println!("{:x}\n", res3);
    println!("The Keccak384 hash for \"Hello\" is:");
    println!("{:x}\n", res4);
    println!("The Keccak512 hash for \"Hello\" is:");
    println!("{:x}\n", res5);
    println!("The Sha3_224 hash for \"Hello\" is:");
    println!("{:x}\n", res6);
    println!("The Sha3_256 hash for \"Hello\" is:");
    println!("{:x}\n", res7);
    println!("The Sha3_384 hash for \"Hello\" is:");
    println!("{:x}\n", res8);
    println!("The Sha3_512 hash for \"Hello\" is:");
    println!("{:x}\n", res9);
*/
}

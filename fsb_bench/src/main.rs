use fsb::{Digest, Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
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

//***************** Fsb160 ***************//

    let now1 = Instant::now();
    for a_string1 in &vec_o_strings {
        let mut hash1 = Fsb160::new();
        hash1.update(a_string1);
        hash1.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later1 = now1.elapsed().subsec_nanos();
    println!("{}", later1);

//******************* Fsb224 *************//

    let now2 = Instant::now();
    for a_string2 in &vec_o_strings {
        let mut hash2 = Fsb224::new();
        hash2.update(a_string2);
        hash2.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later2 = now2.elapsed().subsec_nanos();
    println!("{}", later2);

//******************* Fsb256 *************//

    let now3 = Instant::now();
    for a_string3 in &vec_o_strings {
        let mut hash3 = Fsb256::new();
        hash3.update(a_string3);
        hash3.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later3 = now3.elapsed().subsec_nanos();
    println!("{}", later3);

//******************* Fsb384 *************//

    let now4 = Instant::now();
    for a_string4 in &vec_o_strings {
        let mut hash4 = Fsb384::new();
        hash4.update(a_string4);
        hash4.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later4 = now4.elapsed().subsec_nanos();
    println!("{}", later4);

//******************* Fsb512 *************//

    let now5 = Instant::now();
    for a_string5 in &vec_o_strings {
        let mut hash5 = Fsb512::new();
        hash5.update(a_string5);
        hash5.finalize();
        //let res1 = hash1.finalize();
        //println!("{:x}\n", res1);
    }
    let later5 = now5.elapsed().subsec_nanos();
    println!("{}", later5);
}

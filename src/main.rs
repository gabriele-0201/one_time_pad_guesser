use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input").expect("file not exist");
    let mut lines = String::new();
    file.read_to_string(&mut lines).expect("impossible read");

    let mut lines: Vec<String> = lines.split('\n').map(|s| String::from(s)).collect();
    lines.pop();

    let lines: Vec<Vec<u8>> = lines
        .iter()
        .map(|s| hex::decode(s).expect("Impossibel to decode"))
        .collect();

    let guess_key_and_decode = |input: &Vec<Vec<u8>>,
                                possible_output: Vec<u8>,
                                index_possible_out: u8|
     -> Vec</*output*/ String> {
        let mut key: Vec<u8> = vec![];

        // ciper ^ key = output
        // ciper = output ^ key

        for (index, c) in possible_output.iter().enumerate() {
            key.push(c ^ input[index_possible_out as usize][index]);
        }

        println!(
            "Key used: {}",
            String::from_utf8(key.clone()).expect("Impossible using key not utf8")
        );

        lines
            .iter()
            .map(|s| match String::from_utf8(xor(&key, s)) {
                Ok(decoded) => decoded,
                Err(_) => "NOOP".into(),
            })
            .collect()
    };

    let mut out = String::from("Bitcoin");
    let mut out_index = 2;

    loop {
        let decoded = guess_key_and_decode(&lines, out.as_bytes().to_vec(), out_index);

        for (index, decode) in decoded.iter().enumerate() {
            println!("{} - {:?} \n", index, decode);
        }

        println!("Select input");

        out_index = read_in()
            .parse::<u8>()
            .expect("STUPID, you have to insert a real number");
        println!("Expected output");
        out = read_in();
        println!("Finish LOOOP");
    }
}

fn read_in() -> String {
    let stdin = io::stdin();
    let mut handle;
    let mut in_buffer = String::new();

    handle = stdin.lock();
    handle.read_line(&mut in_buffer).expect("BOH");
    in_buffer.trim().to_string()
}

fn xor(key: &Vec<u8>, value: &Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];

    for i in 0..(value.len() / key.len()) {
        for (index, k) in key.iter().enumerate() {
            if i + index > value.len() {
                return output;
            }
            output.push(k ^ value[i + index]);
        }
    }

    output
}

// key ^ (cyper1 ^ cyper2) = key
//
// input1 ^ cyper2 = key
//
// input1 ^ key = cyper1
// input1 = cyper1 ^ key

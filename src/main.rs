extern crate hex;
#[macro_use] extern crate clap;
extern crate galil_seiferas;

use hex::FromHex;
use std::fs::File;
use std::io::Read;

fn test_xor(needle: &[u8], haystack: &[u8], xor: u8) -> Option<usize> {
    let changed_needle: Vec<u8> = needle.iter().map(|&k| k^xor).collect();
    galil_seiferas::gs_find(haystack, changed_needle.as_ref())
}

fn main() {
    let matches: clap::ArgMatches = clap_app!(xorfinder =>
        (@setting ArgRequiredElseHelp)
        (version: crate_version!())
        (about: "Given a needle and a haystack, find if the needle occurs XORed anywhere in the stack")
        (@arg needle: +required "Needle to find (hex bytes or filename)")
        (@arg haystack: +required "Haystack to find the needle in (hex bytes or filename)")
    ).get_matches();
    let needle = matches.value_of("needle").unwrap();
    let needle: Vec<u8> = match FromHex::from_hex(needle.as_bytes().to_owned()) {
        Ok(result) => result,
        Err(_) => {
            let mut f = File::open(needle).unwrap();
            let mut buf = vec![];
            f.read_to_end(&mut buf);
            buf
        }
    };

    let haystack = matches.value_of("haystack").unwrap();
    let haystack: Vec<u8> = match FromHex::from_hex(haystack.as_bytes().to_owned()) {
        Ok(result) => result,
        Err(_) => {
            let mut f = File::open(haystack).unwrap();
            let mut buf = vec![];
            f.read_to_end(&mut buf);
            buf
        }
    };

    let mut found = false;
    for i in 0u8..255 {
        match test_xor(needle.as_ref(), haystack.as_ref(), i) {
            Some(k) => {
                println!("Found needle in haystack at offset {} XOR {}", k, i);
                found = true;
                break;
            },
            None => continue
        }
    }
    if !found {
        println!("Needle not found in haystack with any XOR value");
    }
}

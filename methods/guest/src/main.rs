#![no_main]
// If you want to try std support, also update the guest Cargo.toml file


use risc0_zkvm::guest::env;
use bls_signatures::{PrivateKey, PublicKey};

risc0_zkvm::guest::entry!(main);


fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let private = PrivateKey::from_string("2523408640174661621672680781521680564539321153090028238087879155142149870407").unwrap();

    let pub_key = private.public_key();
    _ = pub_key;

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}

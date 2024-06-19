#![no_main]
// If you want to try std support, also update the guest Cargo.toml file


use risc0_zkvm::guest::env;
use bls_signatures::{PrivateKey, PublicKey, Serialize};

risc0_zkvm::guest::entry!(main);


fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let private_key = PrivateKey::from_string("2523408640174661621672680781521680564539321153090028238087879155142149870407").unwrap();
    let private_key2 = PrivateKey::from_string("13477572427591894577321901414697637123306849778995612595996293689898472925433").unwrap();

    let message_str = String::from("message");
    let message = message_str.as_bytes();

    let signature = private_key.sign(message);
    let signature2 = private_key2.sign(message);

    let signatures = [signature, signature2];
    let aggregated_signature = bls_signatures::aggregate(&signatures).unwrap();
    let messages = [message, message];    

    let public_key = private_key.public_key();
    let public_key2 = private_key2.public_key();
    
    let public_keys = [public_key, public_key2];
    
    let is_valid = bls_signatures::verify_messages(&aggregated_signature, &messages, &public_keys);
    
    assert!(is_valid);
    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}

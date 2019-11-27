use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::prelude::*;
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};

//#[derive(Debug)]
//struct Account {
    //priv_key: [u8; 32],
    //pub_key: [u8; 64],
    //addr: [u8; 20],
//}

//impl Account {
    //fn new(priv_key: [u8; 32], pub_key: [u8; 64], addr: [u8; 20]) -> Account {
        //Account {
            //priv_key,
            //pub_key,
            //addr,
        //}
    //}
//}

fn gen_account() {
    let mut pk_src: [u8; 32] = [0; 32];
    OsRng.fill_bytes(&mut pk_src);

    let secp = Secp256k1::new();
    let priv_key = SecretKey::from_slice(&pk_src).expect("error generating private_key");
    let pub_key_result = PublicKey::from_secret_key(&secp, &priv_key);
    let pub_key = &pub_key_result.serialize_uncompressed()[1..];

    let mut hasher = Keccak256::new();
    hasher.input(pub_key);
    let address = &hasher.result()[12..];

    println!("private key src: {:?}", pk_src);
    println!("private key: {:?}", priv_key);
    println!("public key: {:?}", pub_key);
    println!("address: {:?}", address);

    //Account::new(pk_src, pub_key, address)
}

fn main() {
    gen_account();
    //let acct = gen_account();
    //println!("{:#?}", acct);
}

use crate::errors;
use bs58;
use chacha20poly1305::aead::stream;
use chacha20poly1305::aead::{Aead, AeadInPlace, NewAead};
use chacha20poly1305::XChaCha20Poly1305;
use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::aes_gcm::AesGcm;
use crypto::chacha20poly1305::ChaCha20Poly1305;
use hex;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::iter::repeat;

pub fn sign(
    tx: String,
    seed_hex: String,
    signature_index: usize,
) -> Result<String, errors::DesoError> {
    // Get our curve set up
    let secp = Secp256k1::new();

    // First get the private key from seed hex
    let private_key = hex::decode(seed_hex).expect("Error decoding seed hex");
    let secret_key = SecretKey::from_slice(&private_key).unwrap();

    // Get transaction bytes
    println!("TX: {}", tx);
    let transaction_bytes = hex::decode(&tx).expect("Problem decoding transaction");
    let v1_fields_buffer = &transaction_bytes[signature_index + 1..];
    let v0_fields_without_signature = &transaction_bytes[0..signature_index];

    // Now double hash the bytes and store in Message struct
    let message = Message::from_hashed_data::<bitcoin_hashes::sha256d::Hash>(&transaction_bytes);

    // Sign the message with the private key
    let signed_sig = secp.sign_ecdsa(&message, &secret_key);

    // Convert to DER
    let serialized_sig = signed_sig.serialize_der();

    // Get the byte array of the signature
    let serialized_bytes = serialized_sig.to_vec();

    // Get the length of the signature
    let length_bytes = usize::to_le_bytes(serialized_bytes.len());
    let length: Vec<u8> = length_bytes
        .iter()
        .copied()
        .filter(|&num| num != 0)
        .collect();

    // // Create new buffer
    let signed_transaction_bytes = [
        v0_fields_without_signature,
        &length,
        &serialized_sig,
        &v1_fields_buffer,
    ]
    .concat();

    let signed_txn_hex = hex::encode(signed_transaction_bytes);

    Ok(signed_txn_hex)
}

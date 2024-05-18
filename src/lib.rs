pub mod crypto_lib;
pub mod errors;
pub mod post_lib;
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionFee {
    #[serde(rename = "PublicKeyBase58Check")]
    recipient_public_key: String,
    #[serde(rename = "AmountNanos")]
    nanos: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraDataBody {
    #[serde(rename = "TransactionHex")]
    transaction_hex: String,
    #[serde(rename = "ExtraData")]
    extra_data: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionHex {
    #[serde(rename = "TransactionHex")]
    transaction_hex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionSubmittedHex {
    #[serde(rename = "TxnHashHex")]
    txn_hash_hex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureIndex {
    #[serde(rename = "SignatureIndex")]
    signature_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTransaction {
    #[serde(rename = "TxnFound")]
    txn_found: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DesoAccount {
    pub name: String,
    pub public_key: String,
    pub seed_hex_key: String,
    pub derived_public_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpatiumAccounts {
    pub accounts: Vec<DesoAccount>,
    pub admin_key: String,
}

pub enum TransactionType {
    POST,
    MINT,
    TRANS,
    ACCEPT,
    PAYMENT,
    ACCEPT_BID,
    MAKE_BID,
    ACCEPT_TRANSFER,
    AUTHORIZE,
    UPDATE,
    ASSOCIATION,
}

const DEBUG: bool = false;

// let transaction_json = create_post(update, &publisher_account, &post_data, client).await?;

pub async fn create_post(
    publisher_account: &DesoAccount,
    post_data: &post_lib::SubmitPostData,
    client: &reqwest::Client,
) -> Result<post_lib::SubmittedTransaction, errors::DesoError> {
    let post_uri = "https://node.deso.org/api/v0/submit-post";

    let post_transaction_response = submit_and_sign(
        post_uri,
        client,
        &post_data,
        1,
        TransactionType::POST,
        publisher_account.seed_hex_key.clone(),
        publisher_account.derived_public_key.clone(),
    )
    .await?;
    let transaction_json: post_lib::SubmittedTransaction =
        match serde_json::from_str(&post_transaction_response.to_string()) {
            Ok(j) => j,
            Err(e) => {
                return Err(errors::DesoError::JsonError(
                    String::from("PUBLISH BOOK"),
                    e.to_string(),
                ))
            }
        };
    let post_hash_hex = transaction_json.post_entry_response.post_hash_hex.clone();

    return Ok(transaction_json);
}

pub async fn get_signature_index(
    tx_hex: &String,
    client: &reqwest::Client,
) -> Result<usize, errors::DesoError> {
    let uri = "https://node.deso.org/api/v0/signature-index";
    let payload = TransactionHex {
        transaction_hex: tx_hex.clone(),
    };
    let resp = match client.post(uri).json(&payload).send().await {
        Ok(s) => s,
        Err(e) => {
            return Err(errors::DesoError::SigningError(
                (format!("Problem getting index response: {}", e.to_string())),
            ));
        }
    };
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    if DEBUG {
        println!("Response: {}", text);
    }
    let json: SignatureIndex = match serde_json::from_str(&text.to_string()) {
        Ok(j) => j,
        Err(e) => {
            return Err(errors::DesoError::SigningError(format!(
                "Problem parsing index response: {}",
                e.to_string()
            )));
        }
    };
    Ok(json.signature_index as usize)
}

pub async fn submit_and_sign<T: Serialize + ?Sized>(
    uri: &str,
    client: &reqwest::Client,
    json: &T,
    retry: u8,
    tx_type: TransactionType,
    signer_hex: String,
    derived_public_key: Option<String>,
) -> Result<String, errors::DesoError> {
    let transaction = match tx_type {
        TransactionType::MINT => "minting",
        TransactionType::TRANS => "transfer",
        TransactionType::POST => "posting",
        TransactionType::ACCEPT => "accepting",
        TransactionType::PAYMENT => "payment",
        TransactionType::ACCEPT_BID => "accepting bid",
        TransactionType::MAKE_BID => "making bid",
        TransactionType::ACCEPT_TRANSFER => "accept transfer",
        TransactionType::AUTHORIZE => "authorizing dervied key",
        TransactionType::UPDATE => "updating nft to be for sale",
        TransactionType::ASSOCIATION => "associating a new author",
    };
    if DEBUG {
        println!("Logging for: {} transaction.", transaction);
    }
    let resp = match client.post(uri).json(&json).send().await {
        Ok(s) => s,
        Err(e) => {
            return Err(errors::DesoError::TransactionError(
                String::from(transaction),
                format!("Error on Post: {}", e.to_string()),
            ));
        }
    };
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    if DEBUG {
        println!("Response: {}", text);
    }
    let json: TransactionHex = match serde_json::from_str(&text.to_string()) {
        Ok(j) => j,
        Err(e) => {
            return Err(errors::DesoError::TransactionError(
                String::from(transaction),
                format!("Problem in Response: {}; {}", text, e.to_string()),
            ))
        }
    };
    println!("BEFORE TX: {}", json.transaction_hex);
    let mut tx_hex = json;
    if let Some(key) = derived_public_key {
        println!("Derived Public Key: {}", key);
        tx_hex = match append_data(&tx_hex, key.to_string(), client).await {
            Ok(t) => t,
            Err(e) => {
                return Err(errors::DesoError::TransactionError(
                    String::from("Error appending derived public key tx"),
                    e.to_string(),
                ));
            }
        };
    }
    if DEBUG {
        println!("\nAfter appending data: {}", tx_hex.transaction_hex);
    }

    // Get signature index
    let signature_index = get_signature_index(&tx_hex.transaction_hex, client).await?;

    let signed_transaction = crypto_lib::sign(tx_hex.transaction_hex, signer_hex, signature_index)?;

    if DEBUG {
        println!("\nAfter signing: {}", signed_transaction);
    }
    let json_transaction_hex: TransactionHex = TransactionHex {
        transaction_hex: signed_transaction,
    };
    let mut i = 0;
    let mut txn_hash_hex: TransactionSubmittedHex = TransactionSubmittedHex {
        txn_hash_hex: String::from(""),
    };

    let mut response_message = String::from("success");

    while i < retry {
        i += 1;
        match submit_transaction(&json_transaction_hex, client).await {
            Ok(s) => {
                response_message = s.clone();
                txn_hash_hex = match serde_json::from_str(&s) {
                    Ok(j) => j,
                    Err(e) => {
                        return Err(errors::DesoError::JsonError(
                            String::from("SUBMIT TX"),
                            e.to_string(),
                        ))
                    }
                };
                break;
            }
            Err(e) => {
                std::thread::sleep(std::time::Duration::from_secs(1 << i));
                println!("Error {}", e.to_string());
            }
        }
    }

    if txn_hash_hex.txn_hash_hex == String::from("") {
        return Err(errors::DesoError::TransactionError(
            String::from(transaction),
            String::from("Transaction Failed :/"),
        ));
    } else if DEBUG {
        println!("Txn Hash Hex: {}", txn_hash_hex.txn_hash_hex);
    }

    // Now we have submitted a transaction successfully, but let's wait and see
    // if it is through before moving on.

    let transaction_check_uri = "https://node.deso.org/api/v0/get-txn";
    let mut pause_count = 0;
    while pause_count < 7 {
        std::thread::sleep(std::time::Duration::from_secs(1 << pause_count));
        match client
            .post(transaction_check_uri)
            .json(&txn_hash_hex)
            .send()
            .await
        {
            Ok(resp) => {
                let text = match resp.text().await {
                    Ok(t) => t,
                    Err(_) => {
                        if DEBUG {
                            println!("ERROR getting response for {}", transaction);
                        }
                        pause_count += 1;
                        continue;
                    }
                };
                let txn_found_struct: GetTransaction = match serde_json::from_str(&text.to_string())
                {
                    Ok(json) => json,
                    Err(_) => {
                        if DEBUG {
                            println!("ERROR in transaction deserialzed for {}", transaction);
                        }
                        pause_count += 1;
                        continue;
                    }
                };
                if txn_found_struct.txn_found {
                    return Ok(response_message);
                } else {
                    pause_count += 1;
                }
            }
            Err(e) => {
                if DEBUG {
                    println!("Error for {}: {}", transaction, e);
                }
                pause_count += 1;
            }
        };
    }
    Ok(response_message)
}

pub async fn get_post_entry_response(
    post_hash_hex: String,
    client: &reqwest::Client,
) -> Result<String, errors::DesoError> {
    let get_single_post_data = post_lib::GetSinglePost {
        post_hash_hex: post_hash_hex,
        comment_limit: 0,
    };
    let uri = "https://node.deso.org/api/v0/get-single-post";
    let resp = match client.post(uri).json(&get_single_post_data).send().await {
        Ok(r) => r,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    Ok(text)
}

pub async fn submit_transaction(
    tx: &TransactionHex,
    client: &reqwest::Client,
) -> Result<String, errors::DesoError> {
    let uri = "https://node.deso.org/api/v0/submit-transaction";
    let resp = match client.post(uri).json(&tx).send().await {
        Ok(r) => r,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    let status: bool = resp.status().is_success();
    let raw_resp = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    println!("Response: {}", status);
    if status {
        Ok(raw_resp)
    } else {
        return Err(errors::DesoError::DesoError(raw_resp));
    }
}

pub async fn append_data(
    tx: &TransactionHex,
    derived_public_key: String,
    client: &reqwest::Client,
) -> Result<TransactionHex, errors::DesoError> {
    let uri = "https://node.deso.org/api/v0/append-extra-data";

    let mut extra_data: HashMap<String, String> = HashMap::new();

    extra_data.insert(String::from("DerivedPublicKey"), derived_public_key);
    let post_data = ExtraDataBody {
        transaction_hex: tx.transaction_hex.clone(),
        extra_data: extra_data,
    };
    let resp = match client.post(uri).json(&post_data).send().await {
        Ok(r) => r,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Err(errors::DesoError::ReqwestError(e.to_string())),
    };
    let json: TransactionHex = match serde_json::from_str(&text.to_string()) {
        Ok(j) => j,
        Err(e) => {
            return Err(errors::DesoError::JsonError(
                String::from("APPEND DATA"),
                e.to_string(),
            ))
        }
    };
    Ok(json)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }
    #[test]
    fn test_create_post() {
        dotenv::from_filename("src/.env").ok();
        let deso_account = env::var("DESO_ACCOUNT").ok();
        let deso_private_key = env::var("PRIVATE_KEY").ok();
        println!("{:?}", env::current_dir());
        let deso_account = DesoAccount {
            name: String::from("Testing"),
            public_key: deso_account.unwrap(),
            seed_hex_key: deso_private_key.unwrap(),
            derived_public_key: None,
        };

        let client = reqwest::Client::new();

        let body = post_lib::SubmitPostBodyObject {
            body: String::from("Testing the new deso rust library by @Spatium!"),
            image_urls: None,
            video_urls: None,
        };

        let mut extra_data_map: HashMap<String, String> = HashMap::new();
        extra_data_map.insert(String::from("nft_type"), String::from("AUTHOR"));

        let post_data = post_lib::SubmitPostData {
            public_key: deso_account.public_key.clone(),
            parent_post_hash_hex: None,
            body_obj: body,
            fee_rate: 1250,
            is_hidden: false,
            extra_data: Some(extra_data_map),
        };

        let post_transaction_json = aw!(create_post(&deso_account, &post_data, &client)).unwrap();

        let post_hash_hex = post_transaction_json.post_entry_response.post_hash_hex;

        let comment_body = post_lib::SubmitPostBodyObject {
            body: String::from("cool comment"),
            image_urls: None,
            video_urls: None,
        };

        let comment_post_data = post_lib::SubmitPostData {
            public_key: deso_account.public_key.clone(),
            parent_post_hash_hex: Some(post_hash_hex),
            body_obj: comment_body,
            fee_rate: 1250,
            is_hidden: false,
            extra_data: None,
        };

        let _comment_transaction_json =
            aw!(create_post(&deso_account, &comment_post_data, &client)).unwrap();
    }
}

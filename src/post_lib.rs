use crate::post_lib;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostEntryResponse {
    #[serde(rename = "PostHashHex")]
    pub post_hash_hex: String, // Hex of the Post Hash. Used as the unique identifier of this post.
    #[serde(rename = "PosterPublicKeyBase58Check")]
    pub poster_public_key: String,
    #[serde(rename = "Body")]
    pub body: String,
    #[serde(rename = "ImageURLs")]
    pub image_urls: Option<Vec<String>>,
    #[serde(rename = "HasUnlockable")]
    pub has_unlockable: bool,
    #[serde(rename = "PostExtraData")]
    pub extra_data: HashMap<String, String>,
    #[serde(rename = "NumNFTCopies")]
    #[serde(default)]
    pub copies_minted: u64,
    #[serde(rename = "TimestampNanos")]
    pub timestamp: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SinglePostResponse {
    #[serde(rename = "PostFound")]
    pub post: PostEntryResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NFTData {
    pub post: PostEntryResponse,
    pub price: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmittedTransaction {
    #[serde(rename = "PostEntryResponse")]
    pub post_entry_response: PostEntryResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostEntryReaderState {
    LikedByReader: bool, // True if the reader has liked this post, otherwise false.
    DiamondLevelBestowed: u8, // Number of diamonds the reader has given this post.
    RepostedByReader: bool, // True if the reader has reposted this post, otherwise false.
    RepostPostHashHex: String, // Hex of the Post Hash in which the user has reposted this post.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPostData {
    #[serde(rename = "UpdaterPublicKeyBase58Check")]
    pub public_key: String,

    #[serde(rename = "ParentStakeID")]
    pub parent_post_hash_hex: Option<String>,

    #[serde(rename = "BodyObj")]
    pub body_obj: SubmitPostBodyObject,

    #[serde(rename = "MinFeeRateNanosPerKB")]
    pub fee_rate: u64,

    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,

    #[serde(rename = "PostExtraData")]
    pub extra_data: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePostData {
    #[serde(rename = "UpdaterPublicKeyBase58Check")]
    pub public_key: String,

    #[serde(rename = "BodyObj")]
    pub body_obj: SubmitPostBodyObject,

    #[serde(rename = "PostHashHexToModify")]
    pub post_hash_hex: String,

    #[serde(rename = "MinFeeRateNanosPerKB")]
    pub fee_rate: u64,

    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,

    #[serde(rename = "PostExtraData")]
    pub extra_data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSinglePost {
    #[serde(rename = "PostHashHex")]
    pub post_hash_hex: String,

    #[serde(rename = "CommentLimit")]
    pub comment_limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPostBodyObject {
    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "ImageURLs")]
    pub image_urls: Option<Vec<String>>,

    #[serde(rename = "VideoURLs")]
    pub video_urls: Option<Vec<String>>,
}

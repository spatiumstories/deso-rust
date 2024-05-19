use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::errors;

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
pub struct SubmittedTransaction {
    #[serde(rename = "PostEntryResponse")]
    pub post_entry_response: PostEntryResponse,
}

/// The main data to post a new post on Deso
#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPostData {
    /// Public key of the user making a new post, editing a post, or making a comment
    #[serde(rename = "UpdaterPublicKeyBase58Check")]
    pub public_key: String,

    /// Only used if making a comment. The post hash hex of the post your commenting on.
    #[serde(rename = "ParentStakeID")]
    pub parent_post_hash_hex: Option<String>,

    /// The body of your post
    #[serde(rename = "BodyObj")]
    pub body_obj: SubmitPostBodyObject,

    /// Min fee rate nanos per kb, defaults to 1250
    #[serde(rename = "MinFeeRateNanosPerKB")]
    pub fee_rate: u64,

    /// Used to "delete" a post. Defaults to false.
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,

    /// An optional map of any meta data for the post
    #[serde(rename = "PostExtraData")]
    pub extra_data: Option<HashMap<String, String>>,
}

/// Builder for building a submit post data
#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPostDataBuilder {
    /// Public key of the user making a new post, editing a post, or making a comment
    pub public_key: Option<String>,

    /// Only used if making a comment. The post hash hex of the post your commenting on.
    pub parent_post_hash_hex: Option<String>,

    /// The body of your post
    pub body: Option<String>,

    /// Any and all images for the post
    pub image_urls: Option<Vec<String>>,

    /// Any and all videos for the post
    pub video_urls: Option<Vec<String>>,

    /// Min fee rate nanos per kb, defaults to 1250
    pub fee_rate: Option<u64>,

    /// Used to "delete" a post, defaults to false
    pub is_hidden: Option<bool>,

    /// An optional map of any meta data for the post
    pub extra_data: Option<HashMap<String, String>>,
}

#[allow(dead_code)]
impl SubmitPostDataBuilder {
    pub fn new() -> Self {
        SubmitPostDataBuilder {
            public_key: None,
            parent_post_hash_hex: None,
            body: None,
            image_urls: None,
            video_urls: None,
            fee_rate: Some(1250),
            is_hidden: Some(false),
            extra_data: None,
        }
    }
    /// Public key of the user making a new post, editing a post, or making a comment
    pub fn public_key(mut self, public_key: String) -> Self {
        self.public_key = Some(public_key);
        self
    }
    /// Only used if making a comment. The post hash hex of the post your commenting on.
    pub fn parent_post_hash_hex(mut self, post_hash_hex: String) -> Self {
        self.parent_post_hash_hex = Some(post_hash_hex);
        self
    }
    /// The body of your post
    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    /// Any and all images for the post
    pub fn image_urls(mut self, image_urls: Vec<String>) -> Self {
        self.image_urls = Some(image_urls);
        self
    }
    /// Any and all videos for the post
    pub fn video_urls(mut self, video_urls: Vec<String>) -> Self {
        self.video_urls = Some(video_urls);
        self
    }
    /// Min fee rate nanos per kb, defaults to 1250
    pub fn fee_rate(mut self, fee_rate: u64) -> Self {
        self.fee_rate = Some(fee_rate);
        self
    }
    /// Used to "delete" a post, defaults to false
    pub fn is_hidden(mut self, is_hidden: bool) -> Self {
        self.is_hidden = Some(is_hidden);
        self
    }
    /// An optional map of any meta data for the post
    pub fn extra_data(mut self, extra_data: HashMap<String, String>) -> Self {
        self.extra_data = Some(extra_data);
        self
    }
    /// Builds the SubmitPostData
    pub fn build(self) -> Result<SubmitPostData, errors::DesoError> {
        if self.body.is_none() {
            return Err(errors::DesoError::SubmitPostDataBuilderError(String::from(
                "Body",
            )));
        }
        if self.public_key.is_none() {
            return Err(errors::DesoError::SubmitPostDataBuilderError(String::from(
                "Poster Public Key",
            )));
        }
        let body_object = SubmitPostBodyObject {
            body: self.body.unwrap(),
            image_urls: self.image_urls,
            video_urls: self.video_urls,
        };
        Ok(SubmitPostData {
            public_key: self.public_key.unwrap(),
            parent_post_hash_hex: self.parent_post_hash_hex,
            body_obj: body_object,
            fee_rate: self.fee_rate.unwrap(),
            is_hidden: self.is_hidden.unwrap(),
            extra_data: self.extra_data,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSinglePost {
    #[serde(rename = "PostHashHex")]
    pub post_hash_hex: String,

    #[serde(rename = "CommentLimit")]
    pub comment_limit: u32,
}

///Body of a Deso post, includes the string content and any images(optional) or videos(optional)
#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPostBodyObject {
    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "ImageURLs")]
    pub image_urls: Option<Vec<String>>,

    #[serde(rename = "VideoURLs")]
    pub video_urls: Option<Vec<String>>,
}

// let body = post_lib::SubmitPostBodyObject {
//     body: String::from("Testing the new deso rust library by @Spatium!"),
//     image_urls: None,
//     video_urls: None,
// };

// let mut extra_data_map: HashMap<String, String> = HashMap::new();
// extra_data_map.insert(String::from("nft_type"), String::from("AUTHOR"));

// let post_data = post_lib::SubmitPostData {
//     public_key: deso_account.public_key.clone(),
//     parent_post_hash_hex: None,
//     body_obj: body,
//     fee_rate: 1250,
//     is_hidden: false,
//     extra_data: Some(extra_data_map),
// };

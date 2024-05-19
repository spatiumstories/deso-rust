# Deso Rust SDK

A Rust SDK for interacting with the Deso blockchain. This library allows you to create posts, NFTs, and messages on the Deso network.

## Overview

The Deso Rust SDK provides a set of utilities to interact with the Deso blockchain, enabling developers to easily perform various blockchain operations, such as creating posts, NFTs, and messages.

## Features

- **Create Posts:** Publish new posts on the Deso blockchain.
- **Create Comments:** Publish new comments on a post

## Create a New Post

To create a new post on the Deso blockchain, follow these steps:

1. **Build the Deso account**: Use the `DesoAccountBuilder` to create your account.

   ```rust
   let deso_account = DesoAccountBuilder::new()
       .public_key(deso_account)
       .seed_hex_key(deso_private_key)
       .build()
       .unwrap();
   ```

2. **Create extra data for the post**: Prepare any additional data you want to include in the post.

   ```rust
   let mut extra_data_map: HashMap<String, String> = HashMap::new();
   extra_data_map.insert(String::from("nft_type"), String::from("AUTHOR"));
   ```

3. **Build the post data**: Use the `SubmitPostDataBuilder` to create the post data.

   ```rust
   let post_data = SubmitPostDataBuilder::new()
       .body(String::from("Testing the new deso rust library by @Spatium!"))
       .public_key(deso_account.public_key.clone())
       .extra_data(extra_data_map)
       .build()
       .unwrap();
   ```

4. **Create the post**: Call the `create_post` function with the prepared data.

   ```rust
   let post_transaction_json = deso::create_post(&deso_account, &post_data).await.unwrap();
   println!("Post created with hash: {:?}", post_transaction_json.post_entry_response.post_hash_hex);
   ```

## Create a Comment on a Post

To create a comment on an existing post, follow these steps:

1. **Build the Deso account**: Use the `DesoAccountBuilder` to create your account.

   ```rust
   let deso_account = DesoAccountBuilder::new()
       .public_key(deso_account)
       .seed_hex_key(deso_private_key)
       .build()
       .unwrap();
   ```

2. **Prepare the post hash hex**: Obtain the hash of the post you want to comment on.

   ```rust
   let post_hash_hex = "existing_post_hash_hex".to_string();
   ```

3. **Build the comment data**: Use the `SubmitPostDataBuilder` to create the comment data.

   ```rust
   let comment_post_data = SubmitPostDataBuilder::new()
       .body(String::from("cool comment bro"))
       .public_key(deso_account.public_key.clone())
       .parent_post_hash_hex(post_hash_hex)
       .build()
       .unwrap();
   ```

4. **Create the comment**: Call the `create_post` function with the prepared data.

   ```rust
   let comment_transaction_json = deso::create_post(&deso_account, &comment_post_data).await.unwrap();
   println!("Comment created with hash: {:?}", comment_transaction_json.post_entry_response.post_hash_hex);
   ```

## To-Do List

- [x] Create Post
- [ ] Create NFT
- [ ] Create Message

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

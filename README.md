# Droplinked-NEAR-Contract
## Introduction
On the droplinked protocol, we are registering products on chain and to enable 3rd party publishers to leverage these registered products and sell them across any marketplace, dapp or native site in order to earn commission. We are complimenting this with headless tooling for NFT Gated store fronts on droplinked.com and other valued added NFT solutions. This particular repository contains the customized contract for the Casper Network.

droplinkeds' contract implements base functionalities of ethereum's ERC-1155 standard. This contract implements SFT tokens (Semi-Fungible Token), which have both uniqueness and value. For example, a producer wants to mint 1M NFTs of the same product (each product has an nft which describes who owns the item); by minting 1M NFT's in a standard such as an ERC-721 (CEP47) is not cost effective (storing 1M ID's and owner address will cost a lot of gas); so instead of minting them one by one, we mint a base token (which contains the ID), and hold that id alongside the number of tokens that a particular account owns. 

This way, we only store a single token ID (which represents the product), and a single number (which represents how many of these token ID's a person owns) for each particular account. 

On droplinked, a publisher can send a publish request to the producer with a particular pre-defined commission amount. The producer can accept or reject requests and if a request is accepted, the publisher is then given the abilkity to publish the product to share with consumers and earn their entitled settlement portion.

## Structure of the contract
Here we explain each structure used within the contract and how they are used:

1. [NFTHolder](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L16-L21) : this struct holds the token ID and its amount for a specific account. remaining_amount is the amount left which is not published for a publisher.
2. [NFTMetadata](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L7-L13) : this struct holds the metadata of a token. It has a name, a URI(it can be IPFS hash), and a checksum (the hash of the file uploaded off-chain), and a price (in USD).
3. [PublishRequest](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L33-L40) : this struct holds the request of a publisher to a producer to publish a token. It has a holder_id, amount, a publisher address, a producer address, and commission. this struct will be saved in a dictionary which maps a request_id to a PublishRequest.
4. [ApprovedNFT](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L23-L31) : this struct holds the data of the approved tokens (for publishers), it has a holder_id, amount, owner and publisher account address, the token_id, and the amount of commission. After approving a PublishRequest by a producer, it will be saved in a dictionary which maps every approved_id to this object.


## Methods
Here we explain each method within the contract and how they are used:

1. [**Mint**](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L95-L149) : gets (`name`,`token_uri`,`checksum`,`price`,`amount`) and mints the `amount` of tokens to `caller`'s account. It first stores the metadata in a `NFTMetadata` struct and saves it in `metadas` dict (which maps a token_id to its `NFTMetadata`). if the `metadata` is already minted, it will use its existing `token_id`. then it will create a `NFTHolder` struct and save it in `holders` dict (which maps a holder_id to a list of `NFTHolder` structs). If the `caller` already owns this token, it will add the `amount` to its `NFTHolder` struct, otherwise it will create a new `NFTHolder` struct and add it to the list.
2. [**publish_request**](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L183-L233) : gets (`producer_account`, `holder_id`, `amount`, `comission`) and creates a `PublishRequest` struct and saves it in `publish_requests` dict (which maps a request_id to a `PublishRequest` struct). Then puts the `request_id` in `producer_requests` dict (which maps a producer account hash to a list of request_ids), also puts the `request_id` in `publisher_requests` dict (which maps a publisher account hash to a list of request_ids). A producer can accept or reject a request and a publisher can cancel any request.
3. [**approve**](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L269-L349) : gets (`request_id`) and approves it, and creates an `ApprovedNFT` struct and saves it in `approved_nfts` dict (which maps a approved_id to an `ApprovedNFT` struct). then puts the `approved_id` in `producer_approved` dict (which maps a producer account hash to a list of approved_ids), also puts the `approved_id` in `publisher_approved` dict (which maps a publisher account hash to a list of approved_ids). A producer can disapprove an approved request at any time post an timestamp.
4. [**disapprove**](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L357-L385) : gets (`approved_id`, `amount`) and disapproves the `approved_id`. If the `amount` is equal to the `amount` of the `ApprovedNFT` struct, it will remove the `approved_id` from `producer_approved` and `publisher_approved` dicts. Otherwise, it will decrease the `amount` of the `ApprovedNFT` struct.
5. [**cancel_request**](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L387-L402) : gets (`request_id`) and removes the `request_id` from `producer_requests` and `publisher_requests` dicts.
6. [**Getter Functions**](https://github.com/FLATLAY/Droplinked-NEAR-Contract/blob/11f6729e439aac45ba8c04289e7508c1316db21f/droplinked-near/src/lib.rs#L151-L181) : These functions are used for interacting with the contract and recieving data.

## Storage Model

![Preview-1](https://user-images.githubusercontent.com/20683538/226114657-869ba832-f870-4571-9692-ff25347f54c3.png)


# Project Feautures
## NFT Gating system
Producers can set a set of rules in order to sell their tokens. They can limit the buyers to accounts which have bought several other tokens by the producer (gating), or they can provide tiered discounts.

These rules (ruleset) are deployed on droplinked.com before the customer purchases the token.

## NFT Storefront
droplinked.com provides a storefront in wich the producers can upload their NFTs and set their prices and rulesets, while customers can explore the NFTs and buy them. These NFT's represent both digital and physical goods.


## Deployed Contract
Account_id for deployed contract : 4bb5d093c0c0e1b4874c41216cdabc5ef1c81c5535b25788202f2a8ce145a7d7

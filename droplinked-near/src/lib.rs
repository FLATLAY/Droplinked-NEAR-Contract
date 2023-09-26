mod event_handler;

use std::fmt::Display;
use std::str::FromStr;
use ed25519_dalek::Verifier;
use event_handler::{MintEvent, DroplinkedEventData, PublishRequestEvent, DisapproveEvent, ApproveEvent};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, PublicKey, Promise};


#[derive(BorshSerialize,BorshDeserialize)]
pub struct NFTMetadata{
    ipfs_url: String,
    commission: u128,
    price: u128
}

#[derive(BorshDeserialize,BorshSerialize)]
pub struct Request{
    token_id: u128,
    producer: AccountId,
    publisher: AccountId,
    accepted: bool
}

fn keys_to_str<V: Display,U:Display>(key1: &V, key2:&U) -> String{
    base16::encode_lower(&env::sha256(format!("{}{}",key1,key2).as_bytes()))
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DroplinkedStorage {
    token_cnts: LookupMap<u128,u128>,
    token_cnt: u128,
    request_cnt: u128,
    total_supply: u128,
    owner: AccountId,
    fee: u128,
    metadata: LookupMap<u128, NFTMetadata>,
    requests: LookupMap<u128, Request>,
    is_requested: LookupMap<String, bool>,
    token_id_by_hash: LookupMap<String, u128>,
    publisher_requests: LookupMap<String, bool>,
    producer_requests: LookupMap<String, bool>,
    uris: LookupMap<u128, String>,
    holders: LookupMap<String, u128>,
    name: String,
    symbol: String,
    heart_beat: u64,
    pub_key: PublicKey
}

impl Default for DroplinkedStorage {
    fn default() -> Self {
        Self {
            token_cnt: 0,
            token_cnts: LookupMap::new(b"a".to_vec()),
            owner : "k3rn3lpanicc.testnet".parse().unwrap(),
            fee: 100,
            metadata : LookupMap::new(b"b".to_vec()),
            total_supply : 0,
            requests : LookupMap::new(b"c".to_vec()),
            producer_requests : LookupMap::new(b"q".to_vec()),
            uris: LookupMap::new(b"d".to_vec()),
            holders : LookupMap::new(b"k".to_vec()),
            token_id_by_hash : LookupMap::new(b"l".to_vec()),
            publisher_requests : LookupMap::new(b"y".to_vec()),
            request_cnt : 0,
            name : "DropNFT".to_string(),
            symbol: "DFT".to_string(),
            heart_beat: 120u64,
            is_requested: LookupMap::new(b"e".to_vec()),
            pub_key: PublicKey::from_str("ed25519:66YNmFT4MhxYBnh8ZEVrC7oJYcr4fqvXwug2oc1AiEB8").unwrap()
        }
    }
}

#[near_bindgen]
impl DroplinkedStorage {
    pub fn get_owner(self) -> AccountId {
        self.owner
    }

    #[payable]
    pub fn mint(&mut self, ipfs_url : String, price : u128,amount : u128, commission: u128) -> u128{
        if env::attached_deposit() < 780000000000000000000{
            env::panic_str("deposit is too low");
        }
        let account_id = env::signer_account_id();
        let metadata_hash = base16::encode_lower(&env::sha256(format!("{}{}{}",ipfs_url,commission,price).as_bytes()));
        let metadata = NFTMetadata{
            ipfs_url: ipfs_url.clone(),
            commission,
            price
        };
        let token_id = {
            if self.token_id_by_hash.contains_key(&metadata_hash){
                let t_id = self.token_id_by_hash.get(&metadata_hash).unwrap();
                self.holders.insert(&keys_to_str(&t_id, &account_id),&amount);
                t_id
            }
            else{
                self.token_cnt+=1;
                self.token_id_by_hash.insert(&metadata_hash,&self.token_cnt);
                self.metadata.insert(&self.token_cnt,&metadata);
                self.uris.insert(&self.token_cnt,&ipfs_url);
                let new_value = self.holders.get(&keys_to_str(&self.token_cnt, &account_id)).unwrap()+amount;
                self.holders.insert(&keys_to_str(&self.token_cnt, &account_id),&new_value);
                self.token_cnt
            }
        };
        match self.token_cnts.get(&token_id){
            None=> {
                self.token_cnts.insert(&token_id,&amount);
            }
            Some(value) =>{
                let new_val = value + amount;
                self.token_cnts.insert(&token_id,&new_val);
            }
        }

        self.total_supply += amount;
        DroplinkedEventData::Mint(MintEvent {token_id , amount, recipient: account_id }).emit();
        token_id
    }


    pub fn get_token_id_by_hash(&self, hash : String) -> Option<u128>{
        self.token_id_by_hash.get(&hash)
    }

    pub fn get_token_hash_by_id(&self, token_id : u128) -> Option<String>{
        let metadata = self.metadata.get(&token_id).unwrap();
        let hash = base16::encode_lower(&&env::sha256(format!("{}{}{}",metadata.ipfs_url,metadata.commission,metadata.price).as_bytes()));
        Some(hash)
    }

    pub fn get_token_metadata(&self, token_id : u128) -> Option<String>{
        let metadata = self.metadata.get(&token_id).unwrap();
        let json = format!(r#"{{"ipfs_url":"{}","commission":"{}","price":{}}}"#,metadata.ipfs_url,metadata.commission,metadata.price);
        Some(json)
    }

    #[payable]
    pub fn publish_request(&mut self, producer_account : AccountId, token_id : u128) -> u128{
        if env::attached_deposit() < 630000000000000000000 {
            env::panic_str("deposit is too low");
        }
        let sender = env::signer_account_id();
        if self.is_requested.get(&keys_to_str(&keys_to_str(&producer_account, &sender), &token_id)).unwrap(){
            env::panic_str("Request Already exists");
        }
        let request_id = self.request_cnt + 1;
        self.request_cnt+=1;
        self.publisher_requests.insert(&keys_to_str(&sender, &request_id), &true);
        self.producer_requests.insert(&keys_to_str(&producer_account, &request_id), &true);
        let req = Request{
            accepted: false,
            producer: producer_account,
            publisher: sender,
            token_id
        };
        self.requests.insert(&request_id, &req);        
        // emit event
        DroplinkedEventData::PublishRequest(PublishRequestEvent { request_id: request_id.to_string(), token_id: token_id.to_string()}).emit();
        request_id
    }

    pub fn get_request(&self, request_id : u128) -> Option<String>{
        let request = self.requests.get(&request_id).unwrap();
        let json = format!(r#"{{"token_id":{},"producer":{},"publisher":{},"accepted":{}}}"#,request.token_id,
        request.producer,request.publisher,request.accepted);
        Some(json)
    }
 
    #[payable]
    pub fn approve(&mut self,request_id : u128){
        if env::attached_deposit() < 770000000000000000000 {
            env::panic_str("deposit is too low");
        }
        let sender = env::signer_account_id();
        if !self.producer_requests.get(&keys_to_str(&sender, &request_id)).unwrap(){
            env::panic_str("Request not found");
        }
        let mut request = self.requests.get(&request_id).unwrap(); 
        request.accepted = true;
        self.requests.insert(&request_id, &request);
        // emit event
        DroplinkedEventData::Approve(ApproveEvent{request_id : request_id.to_string()}).emit();
    }

    #[payable]
    pub fn disapprove(&mut self,request_id : u128){
        // assert!(env::prepaid_gas() > Gas(600_000_000_000_000_000_000));
        
        let sender = env::signer_account_id();
        if sender != self.requests.get(&request_id).unwrap().producer{
            env::panic_str("Access Denied");
        }
        let mut request = self.requests.get(&request_id).unwrap();
        request.accepted = false;
        self.requests.insert(&request_id, &request);
        self.publisher_requests.insert(&keys_to_str(&request.publisher, &request_id), &false);
        self.producer_requests.insert(&keys_to_str(&sender, &request_id), &false);
        self.is_requested.insert(&keys_to_str(&keys_to_str(&request.producer, &request.publisher), &request.token_id), &false);

        // emit event
        DroplinkedEventData::Disapprove(DisapproveEvent {request_id : request_id.to_string()}).emit();
    }
    
    fn verify(&mut self, signature: Vec<u8>, message: &[u8]) {
        let signature = ed25519_dalek::Signature::try_from(signature.as_ref())
            .expect("Signature should be a valid array of 64 bytes [13, 254, 123, ...]");
        // first byte contains CurveType, so we're removing it
        let public_key =
            ed25519_dalek::PublicKey::from_bytes(&self.pub_key.as_bytes()[1..]).unwrap();
        let verification_result = public_key.verify(message, &signature);
        assert!(verification_result.is_ok(), "Invalid signature");
    }

    fn verify_price(&mut self, timestamp : u64, latest_answer: u128, signature : Vec<u8>){
        let message = format!("{},{}", latest_answer,timestamp);
        let hash = env::sha256(message.as_bytes());
        self.verify(signature, hash.as_slice());
    }

    // next function is buy function, it is used to buy tokens from approved holders, it is called by user who wants to buy tokens, it takes approved_id and amount of tokens to buy the "price,timestamp" and a signature of that price and timestamp which is signed by the ratio_verifier account
    // and checks if the signature is true, then gets amount*ratio NEARs from the user and sends them to the producer, and publisher with the comission, and sends the tokens to the user
    #[payable]
    pub fn affiliate_buy(&mut self, amount : u128, shipping: u128, tax: u128, latest_answer: u128, timestamp : u64, signature : Vec<u8>){
        self.verify_price(timestamp.clone(), latest_answer.clone(), signature);
        let request = self.requests.get(&self.request_cnt).unwrap();
        let producer = request.producer;
        let publ = request.publisher;
        let token_id = request.token_id;
        // check the timestamp (needs to be tested!)
        if env::block_timestamp_ms() > timestamp && env::block_timestamp_ms() - timestamp > 2 * self.heart_beat {
            env::panic_str("Old Price");
        }
        let product_price = (self.metadata.get(&token_id).unwrap().price * near_sdk::ONE_NEAR)/latest_answer;
        let total_amount = product_price + (((shipping + tax) * near_sdk::ONE_NEAR)/latest_answer);
        // check the attached deposit
        if env::attached_deposit() < total_amount{
            env::panic_str("deposit is too low");
        }
        // check the amount to buy
        if self.token_cnts.get(&token_id).unwrap() < amount{
            env::panic_str("Not enough tokens");
        }

        // TRANSFER THE NFT NEXT LINE
        // --------------------------
        
        let commission = self.metadata.get(&token_id).unwrap().commission;


        // Transfer the money
        Promise::new(self.owner.clone()).transfer((product_price * self.fee) / 10000u128);
        Promise::new(producer).transfer(total_amount -
            (((product_price * self.fee) / 10000u128) + (((product_price - ((product_price * self.fee) / 10000u128)) *
            commission) / 10000u128)));
        Promise::new(publ).transfer(((product_price - ((product_price * self.fee) / 10000u128)) *
        commission) / 10000u128);
        // EMIT EVENT
    }

    #[payable]
    pub fn recorded_buy(&mut self, producer : AccountId, token_id : u128, shipping: u128, tax: u128, amount : u128, latest_answer: u128, timestamp : u64, signature : Vec<u8>){
        self.verify_price(timestamp.clone(), latest_answer.clone(), signature);
        // check the timestamp (needs to be tested!)
        if env::block_timestamp_ms() > timestamp && env::block_timestamp_ms() - timestamp > 2 * self.heart_beat {
            env::panic_str("Old Price");
        }
        let product_price = (self.metadata.get(&token_id).unwrap().price * near_sdk::ONE_NEAR)/latest_answer;
        let total_amount = product_price + (((shipping + tax) * near_sdk::ONE_NEAR)/latest_answer);
        // check the attached deposit
        if env::attached_deposit() < total_amount{
            env::panic_str("deposit is too low");
        }
        // check the amount to buy
        if self.token_cnts.get(&token_id).unwrap() < amount{
            env::panic_str("Not enough tokens");
        }
        let droplinked_share = (product_price * self.fee) / 10000u128;
        let producer_share = total_amount - droplinked_share;
        Promise::new(self.owner.clone()).transfer(droplinked_share);
        Promise::new(producer).transfer(producer_share);
        // EMIT EVENT
    }
    
    #[payable]
    pub fn direct_buy(&mut self, price : u128, recipient: AccountId, latest_answer: u128, timestamp : u64, signature : Vec<u8>){
        let fee = self.fee;
        self.verify_price(timestamp.clone(), latest_answer.clone(), signature);
        // todo: (Needs to be tested!)
        if env::block_timestamp_ms() > timestamp && env::block_timestamp_ms() - timestamp > 2 * self.heart_beat {
            env::panic_str("Old Price");
        }
        let total_amount = (price * near_sdk::ONE_NEAR)/latest_answer;
        let droplinked_share = (total_amount * fee) / 10000;
        // todo: check the below!
        if env::attached_deposit() < total_amount{
            env::panic_str("deposit is too low");
        }
        Promise::new(recipient).transfer(total_amount - droplinked_share);
        Promise::new(self.owner.clone()).transfer(droplinked_share);
        // EMIT EVENT:
    }
}
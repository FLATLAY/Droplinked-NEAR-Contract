use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::env::block_timestamp_ms;
use near_sdk::{env, near_bindgen, AccountId};


#[derive(BorshSerialize,BorshDeserialize)]
pub struct NFTMetadata{
    name : String,
    token_uri : String,
    checksum : String,
    price : u128
}


#[derive(BorshDeserialize,BorshSerialize)]
pub struct NFTHolder {
    rem_amount : u64,
    amount : u64,
    token_id : u64
}

#[derive(BorshDeserialize,BorshSerialize)]
pub struct ApprovedNFT{
    holder_id : u64,
    amount : u64,
    owner_account : AccountId,
    publisher_account : AccountId,
    token_id : u64,
    comission : u8
}

#[derive(BorshDeserialize,BorshSerialize)]
pub struct PublishRequest{
    holder_id : u64,
    amount : u64,
    comission : u8,
    producer : AccountId,
    publisher : AccountId
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DroplinkedStorage {
    tokens_cnt : u64,
    holders_cnt : u64,
    approved_cnt : u64,
    request_cnt : u64,
    latest_timestamp : u64,
    ratio_verifier : AccountId,
    total_supply : u64,
    requests_objects : LookupMap<u64,PublishRequest>,
    publisher_rejects : LookupMap<AccountId,Vector<u64>>,
    producer_requests : LookupMap<AccountId, Vector<u64>>,
    publisher_requests : LookupMap<AccountId, Vector<u64>>,
    metadatas : LookupMap<u64, NFTMetadata>,
    producer_approved : LookupMap<AccountId,Vector<u64>>,
    publisher_approved : LookupMap<AccountId,Vector<u64>>,
    owners : LookupMap<AccountId,Vector<u64>>,
    holders : LookupMap<u64,NFTHolder>,
    approved : LookupMap<u64,ApprovedNFT>,
    token_id_by_hash : LookupMap<String,u64>
}

impl Default for DroplinkedStorage {
    fn default() -> Self {
        Self {
            tokens_cnt : 0,
            ratio_verifier : "k3rn3lpanicc.testnet".parse().unwrap(),
            metadatas : LookupMap::new(b"m".to_vec()),
            total_supply : 0,
            requests_objects : LookupMap::new(b"r".to_vec()),
            publisher_rejects : LookupMap::new(b"p".to_vec()),
            producer_requests : LookupMap::new(b"q".to_vec()),
            producer_approved : LookupMap::new(b"t".to_vec()),
            publisher_approved : LookupMap::new(b"x".to_vec()),
            owners : LookupMap::new(b"n".to_vec()),
            holders : LookupMap::new(b"k".to_vec()),
            approved : LookupMap::new(b"o".to_vec()),
            token_id_by_hash : LookupMap::new(b"l".to_vec()),
            publisher_requests : LookupMap::new(b"y".to_vec()),
            holders_cnt : 0,
            approved_cnt : 0,
            request_cnt : 0,
            latest_timestamp : 0
        }
    }
}

#[near_bindgen]
impl DroplinkedStorage {
    pub fn get_ratio_verifier(self) -> AccountId{
        self.ratio_verifier
    }
    pub fn mint(&mut self, name : String, token_uri : String, checksum : String, price : u128,amount : u64) -> u64{
        let account_id = env::signer_account_id();
        let metadata_hash = base16::encode_lower(&env::sha256(format!("{}{}{}{}",name,token_uri,checksum,price).as_bytes()));
        let token_id = {
            if self.token_id_by_hash.contains_key(&metadata_hash){
                self.token_id_by_hash.get(&metadata_hash).unwrap()
            }
            else{
                self.tokens_cnt+=1;
                self.token_id_by_hash.insert(&metadata_hash,&self.tokens_cnt);
                self.tokens_cnt
            }
        };
        let metadata = NFTMetadata{
            name,
            token_uri,
            checksum,
            price
        };
        self.metadatas.insert(&token_id,&metadata);

        
        let mut account_holders = self.owners.get(&account_id).unwrap();
        for i in 0..account_holders.len(){
            let mut t = self.holders.get(&account_holders.get(i).unwrap()).unwrap();
            if t.token_id == token_id{
                t.rem_amount += amount;
                t.amount += amount;
                self.holders.insert(&account_holders.get(i).unwrap(),&t);
                return account_holders.get(i).unwrap();
            }
        }
        
        self.holders_cnt += 1;
        let holder_id = self.holders_cnt;
        let holder = NFTHolder{
            rem_amount : amount,
            amount,
            token_id
        };
        
        self.holders.insert(&holder_id,&holder);
        if self.owners.contains_key(&account_id){
            let mut tokens = self.owners.get(&account_id).unwrap();
            tokens.push(&holder_id);
            self.owners.insert(&account_id,&tokens);
        }
        else{
            let mut tokens = Vector::new(format!("n{}",block_timestamp_ms()).as_str().as_bytes().to_vec());
            tokens.push(&holder_id);
            self.owners.insert(&account_id,&tokens);
        }
        self.total_supply += amount;
        holder_id
    }

    pub fn get_owner_tokens(&self, account_id : AccountId) -> Vec<u64>{
        if self.owners.contains_key(&account_id){
            self.owners.get(&account_id).unwrap().to_vec()
        }
        else{
            vec![]
        }
    }

    pub fn get_token_id_by_hash(&self, hash : String) -> Option<u64>{
        self.token_id_by_hash.get(&hash)
    }

    pub fn get_token_hash_by_id(&self, token_id : u64) -> Option<String>{
        let metadata = self.metadatas.get(&token_id).unwrap();
        
        let hash = base16::encode_lower(&env::sha256(format!("{}{}{}{}",metadata.name,metadata.token_uri,metadata.checksum,metadata.price).as_bytes()));
        Some(hash)
    }

    pub fn get_token_metadata(&self, token_id : u64) -> Option<String>{
        let metadata = self.metadatas.get(&token_id).unwrap();
        let json = format!(r#"{{"name":"{}","token_uri":"{}","checksum":"{}","price":{}}}"#,metadata.name,metadata.token_uri,metadata.checksum,metadata.price);
        Some(json)
    }

    pub fn get_holder(&self, holder_id : u64) -> Option<String>{
        let holder = self.holders.get(&holder_id).unwrap();
        let json = format!(r#"{{"rem_amount":{},"amount":{},"token_id":{}}}"#,holder.rem_amount,holder.amount,holder.token_id);
        Some(json)
    }

    pub fn publish_request(&mut self, producer_account : AccountId, amount : u64, holder_id : u64, comission : u8) -> u64{
        let account_id = env::signer_account_id();
        
        //-----------------------Cand be simplified using Maps and Sets-----------------------
        // if producer_account is not owner of holder_id then return error
        let producer_holders = self.owners.get(&producer_account).unwrap();
        let mut is_producer_owner = false;
        for i in 0..producer_holders.len(){
            if producer_holders.get(i).unwrap() == holder_id{
                is_producer_owner = true;
                break;
            }
        }
        if !is_producer_owner{
            env::panic_str("Producer is not owner of holder_id");
        }
        //-----------------------------------------------------------------------------------
        
        let request_id = self.request_cnt + 1;
        let request = PublishRequest{
            amount,
            holder_id,
            comission,
            producer : producer_account.clone(),
            publisher : account_id.clone()
        };
        self.requests_objects.insert(&request_id,&request);
        self.request_cnt = request_id;
        // add request_id to publisher_requests and producer_requests
        if self.publisher_requests.contains_key(&account_id){
            let mut requests = self.publisher_requests.get(&account_id).unwrap();
            requests.push(&request_id);
            self.publisher_requests.insert(&account_id,&requests);
        }
        else{
            let mut requests = Vector::new(format!("y{}",block_timestamp_ms()).as_str().as_bytes().to_vec());
            requests.push(&request_id);
            self.publisher_requests.insert(&account_id,&requests);
        }
        if self.producer_requests.contains_key(&producer_account){
            let mut requests = self.producer_requests.get(&producer_account).unwrap();
            requests.push(&request_id);
            self.producer_requests.insert(&producer_account,&requests);
        }
        else{
            let mut requests = Vector::new(format!("z{}",block_timestamp_ms()).as_str().as_bytes().to_vec());
            requests.push(&request_id);
            self.producer_requests.insert(&producer_account,&requests);
        }
        request_id       
    }

    pub fn get_request(&self, request_id : u64) -> Option<String>{
        let request = self.requests_objects.get(&request_id).unwrap();
        let json = format!(r#"{{"amount":{},"holder_id":{},"comission":{},"producer":"{}","publisher":"{}"}}"#,request.amount,request.holder_id,request.comission,request.producer,request.publisher);
        Some(json)
    }

    pub fn get_publisher_requests(&self, publisher_account : AccountId) -> Option<String>{
        let mut json = String::from("[");
        if self.publisher_requests.contains_key(&publisher_account){
            let requests = self.publisher_requests.get(&publisher_account).unwrap();
            for request_id in requests.iter(){
                let request = self.requests_objects.get(&request_id).unwrap();
                let json_part = format!(r#"{{"request_id":{},"amount":{},"holder_id":{},"comission":{},"producer":"{}","publisher":"{}"}},"#,request_id,request.amount,request.holder_id,request.comission,request.producer,request.publisher);
                json.push_str(&json_part);
            }
        }
        json.push_str("]");
        Some(json)
    }

    pub fn get_producer_requests(&self, producer_account : AccountId) -> Option<String>{
        let mut json = String::from("[");
        if self.producer_requests.contains_key(&producer_account){
            let requests = self.producer_requests.get(&producer_account).unwrap();
            for request_id in requests.iter(){
                let request = self.requests_objects.get(&request_id).unwrap();
                let json_part = format!(r#"{{"request_id":{},"amount":{},"holder_id":{},"comission":{},"producer":"{}","publisher":"{}"}},"#,request_id,request.amount,request.holder_id,request.comission,request.producer,request.publisher);
                json.push_str(&json_part);
            }
        }
        json.push_str("]");
        Some(json)
    }

    pub fn approve(&mut self,request_id : u64) -> u64{
        let account_id = env::signer_account_id();
        let request_wrapped = self.requests_objects.get(&request_id);
        if request_wrapped.is_none(){
            env::panic_str("Request does not exist");
        }
        let request = request_wrapped.unwrap();
        if request.producer != account_id{
            env::panic_str("Caller is not producer");
        }
        
        // check if caller owns holder_id
        let producer_holders = self.owners.get(&account_id).unwrap();
        let mut is_producer_owner = false;
                
        for i in 0..producer_holders.len(){
            if producer_holders.get(i).unwrap() == request.holder_id{
                is_producer_owner = true;
                break;
            }
        }
        if !is_producer_owner{
            env::panic_str("Producer is not owner of holder_id");
        }

        let mut holder = self.holders.get(&request.holder_id).unwrap();
        if holder.rem_amount < request.amount{
            env::panic_str("Not enough tokens in holder");
        }
        holder.rem_amount -= request.amount;
        let approved_holder = ApprovedNFT {
            holder_id : request.holder_id,
            amount : request.amount,
            comission : request.comission,
            owner_account : request.producer.clone(),
            publisher_account : request.publisher.clone(),
            token_id : holder.token_id
        };
        self.holders.insert(&request.holder_id,&holder);


        // get approved cnt and increment it
        self.approved_cnt += 1;
        let approved_id = self.approved_cnt;
        self.approved.insert(&approved_id,&approved_holder);

        //add the approved holder to the publishers approved dictionary
        if self.publisher_approved.contains_key(&request.publisher){
            let mut approved = self.publisher_approved.get(&request.publisher).unwrap();
            approved.push(&approved_id);
            self.publisher_approved.insert(&request.publisher,&approved);
        }
        else{
            let mut approved = Vector::new(format!("x{}",block_timestamp_ms()).as_str().as_bytes().to_vec());
            approved.push(&approved_id);
            self.publisher_approved.insert(&request.publisher,&approved);
        }
        //add the approved holder to the producers approved dictionary
        if self.producer_approved.contains_key(&request.producer){
            let mut approved = self.producer_approved.get(&request.producer).unwrap();
            approved.push(&approved_id);
            self.producer_approved.insert(&request.producer,&approved);
        }
        else{
            let mut approved = Vector::new(format!("y{}",block_timestamp_ms()).as_str().as_bytes().to_vec());
            approved.push(&approved_id);
            self.producer_approved.insert(&request.producer,&approved);
        }
        //remove the request from the publishers requests dictionary and the producers requests dictionary
        let mut publisher_requests = self.publisher_requests.get(&request.publisher).unwrap();
        let index_of_request = publisher_requests.iter().position(|x| x == request_id).unwrap();
        publisher_requests.swap_remove(index_of_request as u64);
        self.publisher_requests.insert(&request.publisher,&publisher_requests);
        let mut producer_requests = self.producer_requests.get(&request.producer).unwrap();
        let index_of_request = producer_requests.iter().position(|x| x == request_id).unwrap();
        producer_requests.swap_remove(index_of_request as u64);
        self.producer_requests.insert(&request.producer,&producer_requests);
        //remove the request from the requests_objects dictionary
        self.requests_objects.remove(&request_id);
        approved_id
    }
    
    pub fn get_approved(&self, approved_id : u64) -> Option<String>{
        let approved = self.approved.get(&approved_id).unwrap();
        let json = format!(r#"{{"approved_id":{},"holder_id":{},"amount":{},"comission":{},"owner_account":"{}","publisher_account":"{}","token_id":{}}}"#,approved_id,approved.holder_id,approved.amount,approved.comission,approved.owner_account,approved.publisher_account,approved.token_id);
        Some(json)
    }

    pub fn disapprove(&mut self,approved_id : u64, amount : u64){
        let account_id = env::signer_account_id();
        let approved = self.approved.get(&approved_id).unwrap();
        if approved.owner_account != account_id{
            env::panic_str("Caller is not owner");
        }
        if approved.amount < amount{
            env::panic_str("Not enough tokens in approved");
        }
        let mut holder = self.holders.get(&approved.holder_id).unwrap();
        holder.rem_amount += amount;
        self.holders.insert(&approved.holder_id,&holder);
        if approved.amount == amount{
            self.approved.remove(&approved_id);
            let mut producer_approved = self.producer_approved.get(&approved.owner_account).unwrap();
            let index_of_approved = producer_approved.iter().position(|x| x == approved_id).unwrap();
            producer_approved.swap_remove(index_of_approved as u64);
            self.producer_approved.insert(&approved.owner_account,&producer_approved);
            let mut publisher_approved = self.publisher_approved.get(&approved.publisher_account).unwrap();
            let index_of_approved = publisher_approved.iter().position(|x| x == approved_id).unwrap();
            publisher_approved.swap_remove(index_of_approved as u64);
            self.publisher_approved.insert(&approved.publisher_account,&publisher_approved);
        }
        else{
            let mut approved = self.approved.get(&approved_id).unwrap();
            approved.amount -= amount;
            self.approved.insert(&approved_id,&approved);
        }
    }

    pub fn cancel_request(&mut self,request_id : u64){
        let account_id = env::signer_account_id();
        let request = self.requests_objects.get(&request_id).unwrap();
        if request.publisher != account_id{
            env::panic_str("Caller is not producer");
        }
        let mut producer_requests = self.producer_requests.get(&request.producer).unwrap();
        let index_of_request = producer_requests.iter().position(|x| x == request_id).unwrap();
        producer_requests.swap_remove(index_of_request as u64);
        self.producer_requests.insert(&request.producer,&producer_requests);
        let mut publisher_requests = self.publisher_requests.get(&request.publisher).unwrap();
        let index_of_request = publisher_requests.iter().position(|x| x == request_id).unwrap();
        publisher_requests.swap_remove(index_of_request as u64);
        self.publisher_requests.insert(&request.publisher,&publisher_requests);
        self.requests_objects.remove(&request_id);
    }

    pub fn producers_approved(&self, producer_account : AccountId) -> Option<Vec<u64>>{
        Some(self.producer_approved.get(&producer_account).unwrap().to_vec())
    }
    pub fn publishers_approved(&self, publisher_account : AccountId) -> Option<Vec<u64>>{
        Some(self.publisher_approved.get(&publisher_account).unwrap().to_vec())
    }
    
    // next function is buy function, it is used to buy tokens from approved holders, it is called by user who wants to buy tokens, it takes approved_id and amount of tokens to buy the "price,timestamp" and a signature of that price and timestamp which is signed by the ratio_verifier account
    // and checks if the signature is true, then gets amount*ratio NEARs from the user and sends them to the producer, and publisher with the comission, and sends the tokens to the user
    // #[payable]
    // pub fn buy(&mut self, approved_id : u64, amount : u64, price : u128, timestamp : u64, signature : String){
    //     let account_id = env::signer_account_id();
    //     let approved = self.approved.get(&approved_id).unwrap();
    //     if approved.amount < amount{
    //         env::panic_str("Not enough tokens in approved");
    //     }
    //     let mut holder = self.holders.get(&approved.holder_id).unwrap();
        
    //     // TODO: check the signature
        
        

    // }
}
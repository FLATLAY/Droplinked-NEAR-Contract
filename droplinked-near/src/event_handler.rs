use near_sdk::{AccountId, env};


pub (crate) fn log(message: &str) {
    env::log_str(message);
}

pub struct MintEvent {
    pub token_id: u128,
    pub recipient: AccountId,
    pub amount: u128
}

pub struct PublishRequestEvent {
    pub request_id : String,
    pub token_id : String,
}

pub struct ApproveEvent {
    pub request_id : String,
}

pub struct DisapproveEvent {
    pub request_id : String
}
// It should contain Mint, PublishRequest, Approve, Disapprove and Cancel event
pub enum DroplinkedEventData{
    Mint(MintEvent),
    PublishRequest(PublishRequestEvent),
    Approve(ApproveEvent),
    Disapprove(DisapproveEvent),
}

impl DroplinkedEventData{
    pub fn emit(&self){
        match self {
            DroplinkedEventData::Mint(mint_event) => {
                log(&format!("{{\"event_kind\":\"Mint\",\"token_id\":\"{}\",\"recipient\":\"{}\",\"amount\":\"{}\"}}", mint_event.token_id, mint_event.recipient, mint_event.amount));
            },
            DroplinkedEventData::PublishRequest(publish_request_event) => {
                log(&format!("{{\"event_kind\":\"PublishRequest\",\"token_id\":\"{}\",\"request_id\":\"{}\"}}", publish_request_event.token_id, publish_request_event.request_id));
            },
            DroplinkedEventData::Approve(approve_event) => {
                log(&format!("{{\"event_kind\":\"Approve\",\"request_id\":\"{}\"}}", approve_event.request_id));
            },
            DroplinkedEventData::Disapprove(disapprove_event) => {
                log(&format!("{{\"event_kind\":\"Disapprove\",\"request_id\":\"{}\"}}", disapprove_event.request_id));
            }
        }
    }    
}



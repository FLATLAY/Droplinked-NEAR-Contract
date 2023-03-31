use near_sdk::env;


pub (crate) fn log(message: &str) {
    env::log_str(message);
}

pub struct MintEvent {
    pub token_id: String,
    pub holder_id: String,
    pub owner_id: String,
}

pub struct PublishRequestEvent {
    pub request_id : String,
    pub holder_id : String,
    pub owner_id : String,
    pub commission : String,
    pub publisher_id : String,
}

pub struct ApproveEvent {
    pub request_id : String,
    pub holder_id : String,
    pub owner_id : String,
    pub approved_id : String,
}

pub struct DisapproveEvent {
    pub approved_id : String,
    pub holder_id : String,
    pub owner_id : String,
}

pub struct CancelEvent {
    pub request_id : String,
    pub holder_id : String,
    pub owner_id : String,
}

// It should contain Mint, PublishRequest, Approve, Disapprove and Cancel event
pub enum DroplinkedEventData{
    Mint(MintEvent),
    PublishRequest(PublishRequestEvent),
    Approve(ApproveEvent),
    Disapprove(DisapproveEvent),
    Cancel(CancelEvent),
}

impl DroplinkedEventData{
    pub fn emit(&self){
        match self {
            DroplinkedEventData::Mint(mint_event) => {
                log(&format!("{{\"event_kind\":\"Mint\",\"token_id\":\"{}\",\"holder_id\":\"{}\",\"owner_id\":\"{}\"}}", mint_event.token_id, mint_event.holder_id, mint_event.owner_id));
            },
            DroplinkedEventData::PublishRequest(publish_request_event) => {
                log(&format!("{{\"event_kind\":\"PublishRequest\",\"request_id\":\"{}\",\"holder_id\":\"{}\",\"owner_id\":\"{}\",\"commission\":\"{}\",\"publisher_id\":\"{}\"}}", publish_request_event.request_id, publish_request_event.holder_id, publish_request_event.owner_id, publish_request_event.commission, publish_request_event.publisher_id));
            },
            DroplinkedEventData::Approve(approve_event) => {
                log(&format!("{{\"event_kind\":\"Approve\",\"request_id\":\"{}\",\"holder_id\":\"{}\",\"owner_id\":\"{}\",\"approved_id\":\"{}\"}}", approve_event.request_id, approve_event.holder_id, approve_event.owner_id, approve_event.approved_id));
            },
            DroplinkedEventData::Disapprove(disapprove_event) => {
                log(&format!("{{\"event_kind\":\"Disapprove\",\"approved_id\":\"{}\",\"holder_id\":\"{}\",\"owner_id\":\"{}\"}}", disapprove_event.approved_id, disapprove_event.holder_id, disapprove_event.owner_id));
            },
            DroplinkedEventData::Cancel(cancel_event) => {
                log(&format!("{{\"event_kind\":\"Cancel\",\"request_id\":\"{}\",\"holder_id\":\"{}\",\"owner_id\":\"{}\"}}", cancel_event.request_id, cancel_event.holder_id, cancel_event.owner_id));
            },
        }
    }    
}



use serde::{Serialize, Deserialize};
use crate::model::L1;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerOut {
    pub jsonrpc: String,
    pub method: String,
    pub params: Params,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub channel: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub timestamp: i64,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "best_bid_price")]
    pub best_bid_price: f64,
    #[serde(rename = "best_bid_amount")]
    pub best_bid_amount: f64,
    #[serde(rename = "best_ask_price")]
    pub best_ask_price: f64,
    #[serde(rename = "best_ask_amount")]
    pub best_ask_amount: f64,
}

impl DerOut {
    pub fn to_l1(&self) -> L1 {
        
        L1 {
            bid_p: self.params.data.best_bid_price as f32,
            bid_q: self.params.data.best_bid_amount as f32,
            ask_p: self.params.data.best_ask_price as f32,
            ask_q: self.params.data.best_ask_amount as f32,
        }
    }
}
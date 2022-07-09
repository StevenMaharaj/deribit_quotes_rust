// mod deribit_out;
// use serde_derive::Deserialize;
// use serde_derive::Serialize;
use serde::{Serialize, Deserialize};
// use serde_json;
// 

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub jsonrpc: String,
    pub method: String,
    pub id: i64,
    pub params: Params,
}

#[derive( Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub channels: Vec<String>,
}


pub fn make_quote_message(products: Vec<String>) -> Root {
    let params =  Params {
        channels: products
    };
    let deribit_in = Root {
        jsonrpc: String::from("2.0"),
        method: String::from("public/subscribe"),
        id:42,
        params,
    };

    deribit_in
}




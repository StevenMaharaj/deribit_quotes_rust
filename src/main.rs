mod derbit_out_put;
mod deribit_out;
mod model;
use serde_json;
use tungstenite::{connect, Message};
use url::Url;
use derbit_out_put::DerOut;


fn main() {
    // Connect to the WS server locally
    let products: Vec<String> = vec![
        "quote.BTC-PERPETUAL".to_string(),

    ];
    let msg = deribit_out::make_quote_message(products);
    let (mut socket, _response) =
        connect(Url::parse("wss://www.deribit.com/ws/api/v2").unwrap()).expect("Can't connect"); // Write a message containing "Hello, Test!" to the server
    socket
        .write_message(Message::Text(serde_json::to_string(&msg).unwrap()))
        .unwrap();

    // Loop forever, handling parsing each message
    let mut msg = socket.read_message().expect("Error reading message");
    println!("{}", msg);
  

    let mut res: DerOut;
    loop {
        msg = socket.read_message().expect("Error reading message");
        let msg_str = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!()
            }
        };
        // println!("{}", msg);
        res = serde_json::from_str(&msg_str).unwrap();
        
        println!("{:?}", res.to_l1().ask_q);
        
    }
}

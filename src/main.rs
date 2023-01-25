use std::env;
use reqwest;
use serde_json::json;
use serde_json::Value;


#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
 

    let args: Vec<String> = env::args().collect();

    let echo_json: Value = reqwest::Client::new()
        .get("https://www.att.com/services/shop/model/ecom/shop/view/unified/qualification/service/CheckAvailabilityRESTService/invokeCheckAvailability")
        .query(&[("userInputZip",args[1].to_string()), 
                ("userInputAddressLine1", args[2].to_string()),
                ("mode", "fullAddress".to_string())])
        .header("Accept", "*/*")
        .header("Connection", "keep-alive")
        .header("Content-Type", "Application/Json")
        .send()
        .await?
        .json()
        .await?;

    println!("{:?}", json!(echo_json).get("profile").expect("Not found").get("isGIGAFiberAvailable").unwrap().as_bool().unwrap()
);


    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

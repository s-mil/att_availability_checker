use std::env;
use reqwest;


#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
 

    let args: Vec<String> = env::args().collect();
    dbg!("{:?}",&args);
    let echo_json = reqwest::Client::new()
        .get("https://www.att.com/services/shop/model/ecom/shop/view/unified/qualification/service/CheckAvailabilityRESTService/invokeCheckAvailability")
        .query(&[("userInputZip","774229"), 
                ("userInputAddressLine1", "14307 Cypress Valley Drive"),
                ("mode", "fullAddress")])
        .header("Accept", "*/*")
        .header("Connection", "keep-alive")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Content-Type", "Application/Json")
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", echo_json);


    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

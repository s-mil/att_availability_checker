use reqwest;
use serde_json::json;
use serde_json::Value;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Zip code of address

    zip: Option<String>,

    /// Street address "123 main"
  
    street: Option<String>,


}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
    

    let args = Args::parse();

    let echo_json: Value = reqwest::Client::new()
        .get("https://www.att.com/services/shop/model/ecom/shop/view/unified/qualification/service/CheckAvailabilityRESTService/invokeCheckAvailability")
        .query(&[("userInputZip", args.zip.expect("need zip")), 
                ("userInputAddressLine1", args.street.expect("Need street address")),
                ("mode", "fullAddress".to_string())])
        .header("Accept", "*/*")
        .header("Connection", "keep-alive")
        .header("Content-Type", "Application/Json")
        .send()
        .await?
        .json()
        .await?;

    let fiber: bool = json!(echo_json).get("profile").expect("Profile Not found").get("isGIGAFiberAvailable").expect("didnt find the GigaFiber").as_bool().expect("It didnt parse the json right");
    println!("{}", &fiber);


    return Ok(());
}

#[cfg(target_arch = "wasm32")]
fn main() {}

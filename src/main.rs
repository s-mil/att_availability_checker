use clap::Parser;
use reqwest;
use serde_json::json;
use serde_json::Value;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Zip code of address
    zip: Option<String>,

    /// Street address without type "123 main"
    street: Option<String>,

    /// Optional, set to file path of json to parse for checking multiple addresses overrides zip and street options
    #[clap(short, long)]
    json_file: Option<String>,
}

fn load_file(file_name: &str) -> serde_json::Value {
    let file_content: String = fs::read_to_string(file_name).expect("error reading file");
    serde_json::from_str::<Value>(&file_content).expect("error serializing to JSON")
}

async fn get_att_response(
    client: reqwest::RequestBuilder,
    zip: &str,
    street: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let response: serde_json::Value = client
        .query(&[
            ("userInputZip", zip),
            ("userInputAddressLine1", street),
            ("mode", "fullAddress"),
        ])
        .header("Accept", "*/*")
        .header("Connection", "keep-alive")
        .header("Content-Type", "Application/Json")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let att_client: reqwest::RequestBuilder = reqwest::Client::new()
    .get("https://www.att.com/services/shop/model/ecom/shop/view/unified/qualification/service/CheckAvailabilityRESTService/invokeCheckAvailability");

    if args.json_file.is_some() {
        let json_data = load_file(&args.json_file.expect("file not found"));
        
        if let Some(items) =  json_data.as_array() {
            items.iter_mut()
            .map(|item| tokio::spawn(item.resolve()))
            .collect();
            
            for item in &items{
                item.print_result()
            }
        }

        println!("{:#?}", serde_json::to_string(&json_data).unwrap());


    } else {
        let att_response: Value = get_att_response(att_client, &args.zip.expect("some string zip"), &args.street.expect("some string street")).await?;

        let fiber: bool = json!(att_response)
            .get("profile")
            .expect("Profile Not found")
            .get("isGIGAFiberAvailable")
            .expect("didnt find the GigaFiber")
            .as_bool()
            .expect("It didnt parse the json right");
        println!("{}", &fiber);
    }
    return Ok(());
}

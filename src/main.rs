use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Shipment {
    #[serde(rename = "shipmentId")]
    id: String,
    #[serde(rename = "orderDate")]
    order_date: String,
    #[serde(rename = "deliveryDate")]
    delivery_date: String,
    #[serde(rename = "isDelayed")]
    delayed: bool,
    #[serde(rename = "trackingInfo")]
    tracking_info: TrackingInfo,
    #[serde(rename = "address")]
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Deserialize)]
struct TrackingInfo {
    carrier: Option<String>,
    #[serde(rename = "expectedTime")]
    expected_time: Option<String>,
    status: String,
    #[serde(rename = "trackingNumber")]
    tracking_number: String,
    #[serde(rename = "trackingLink")]
    tracking_link: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.mattressfirm.com/track/_next/data/1PV9Lq-1gfywGk2GkInta/results.json?orderNumber=&formZipCode=";
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0";

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header(USER_AGENT, user_agent).send()?;
    let data = response.text()?;

    let props: serde_json::Value = serde_json::from_str(&data).unwrap();

    let shipment: Shipment =
        serde_json::from_value(props["pageProps"]["data"]["shipments"][0].clone())?;

    Ok(())
}

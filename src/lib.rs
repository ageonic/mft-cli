#[macro_use]
extern crate prettytable;

use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::error::Error;

const BASE_URL: &str =
    "https://www.mattressfirm.com/track/_next/data/1PV9Lq-1gfywGk2GkInta/results.json";
const USER_AGENT_VALUE: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0";

pub struct Tracker {
    order_number: String,
    form_secondary_detail_name: String,
    form_secondary_detail_value: String,
}

impl Tracker {
    pub fn new(order_number: String, verification_detail: Verification) -> Tracker {
        let (form_secondary_detail_name, form_secondary_detail_value) = match verification_detail {
            Verification::LastName(last_name) => (String::from("formLastName"), last_name),
            Verification::ZipCode(zip) => (String::from("formZipCode"), zip),
            Verification::Phone(phone) => (String::from("formPhone"), phone),
        };
        Tracker {
            order_number,
            form_secondary_detail_name,
            form_secondary_detail_value,
        }
    }

    pub fn get_shipment_details(&self) -> Result<Shipment, Box<dyn Error>> {
        let url = format!(
            "{}?orderNumber={}&{}={}",
            BASE_URL,
            self.order_number,
            self.form_secondary_detail_name,
            self.form_secondary_detail_value
        );

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .header(USER_AGENT, USER_AGENT_VALUE)
            .send()?;
        let data = response.text()?;

        let props: serde_json::Value = serde_json::from_str(&data).unwrap();

        let shipment: Shipment =
            serde_json::from_value(props["pageProps"]["data"]["shipments"][0].clone())?;

        Ok(shipment)
    }
}

pub enum Verification {
    LastName(String),
    ZipCode(String),
    Phone(String),
}

#[derive(Debug, Deserialize)]
pub struct Shipment {
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
pub struct TrackingInfo {
    carrier: Option<String>,
    #[serde(rename = "expectedTime")]
    expected_time: Option<String>,
    status: String,
    #[serde(rename = "trackingNumber")]
    tracking_number: String,
    #[serde(rename = "trackingLink")]
    tracking_link: String,
}

impl Shipment {
    pub fn show(&self) {
        let table = table!(
            [
                "Delivery date",
                "Delayed",
                "Tracking number",
                "Tracking URL"
            ],
            [
                self.delivery_date,
                self.delayed,
                self.tracking_info.tracking_number,
                self.tracking_info.tracking_link
            ]
        );

        table.printstd();
    }

    pub fn show_verbose(&self) {
        let table = table!(
            [
                "Shipment ID",
                "Delivery date",
                "Expected time",
                "Delayed",
                "Carrier",
                "Status",
                "Tracking number",
                "Tracking URL"
            ],
            [
                self.id,
                self.delivery_date,
                self.tracking_info
                    .expected_time
                    .as_ref()
                    .unwrap_or(&String::from("-")),
                self.delayed,
                self.tracking_info
                    .carrier
                    .as_ref()
                    .unwrap_or(&String::from("-")),
                self.tracking_info.status,
                self.tracking_info.tracking_number,
                self.tracking_info.tracking_link
            ]
        );

        table.printstd();
    }
}

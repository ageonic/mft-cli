#[macro_use]
extern crate prettytable;

use serde::Deserialize;

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

    fn show_verbose(&self) {
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

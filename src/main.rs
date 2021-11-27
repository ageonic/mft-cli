use clap::{App, Arg};
use mft::Verification;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Mattress Firm Delivery Tracker")
        .version("1.0")
        .author("Aaron G. <aarongeoag@gmail.com>")
        .about("CLI to retrieve tracking information for Mattress Firm orders.")
        .arg(
            Arg::with_name("order_number")
                .short("o")
                .long("ordernumber")
                .value_name("ORDER_NUMBER")
                .help("The order number from your receipt")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("zip")
                .short("z")
                .long("zip")
                .value_name("ZIP")
                .help("The zip code associated with the order")
                .takes_value(true)
                .required(true)
                .conflicts_with_all(&["last_name", "phone"]),
        )
        .arg(
            Arg::with_name("phone")
                .short("p")
                .long("phone")
                .value_name("PHONE")
                .help("The phone number associated with the order")
                .takes_value(true)
                .required(true)
                .conflicts_with_all(&["last_name", "zip"]),
        )
        .arg(
            Arg::with_name("last_name")
                .short("n")
                .long("lastname")
                .value_name("LAST_NAME")
                .help("The last name associated with the order")
                .takes_value(true)
                .required(true)
                .conflicts_with_all(&["zip", "phone"]),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Show more order details"),
        )
        .get_matches();

    let order_number = String::from(matches.value_of("order_number").unwrap());
    let secondary_detail: Verification;

    if matches.is_present("zip") {
        secondary_detail = Verification::ZipCode(String::from(matches.value_of("zip").unwrap()));
    } else if matches.is_present("phone") {
        secondary_detail = Verification::Phone(String::from(matches.value_of("phone").unwrap()));
    } else {
        // last name will definitely be provided if zip and phone are not
        secondary_detail =
            Verification::LastName(String::from(matches.value_of("last_name").unwrap()));
    }

    let tracker = mft::Tracker::new(order_number, secondary_detail);
    let shipments = tracker.get_shipment_details()?;

    match matches.occurrences_of("verbose") {
        0 => shipments.show(),
        1 => shipments.show_verbose(),
        _ => shipments.show_verbose(),
    }

    Ok(())
}

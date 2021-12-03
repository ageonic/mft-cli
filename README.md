# Mattress Firm CLI Delivery Tracker

A simple CLI to retrieve and display delivery information for mattress firm orders. This project was created as an exercise in exploring the Rust programming language.

## Installation

Clone this repository:

    $ git clone https://github.com/ageonic/mft-cli.git
    $ cd mft-cli

Then build the project using `cargo`:

    $ cargo build

The resulting `mft` binary will be in `target/release/`. Copy the binary to `/usr/local/bin` (linux) or any other directory that is on your local PATH. This will enable you to use the `mft` command from outside the project directory.

    $ cp target/release/mft /usr/local/bin/

Confirm that the installation was successful by invoking the command:

    $ mft --version
    Mattress Firm Delivery Tracker 1.0
    
## Usage

`mft` requires an order number and the zip code (or last name) associated with the order to display tracking details:

    $ mft -o SOME_ORDER_NUMBER -z ZIP_CODE
    +---------------+---------+--------------------+
    | Delivery date | Delayed | Tracking number    |
    +---------------+---------+--------------------+
    | Feb 30, 2025  | false   | longtrackingnumber |
    +---------------+---------+--------------------+

    Tracking URL: UNAVAILABLE

Make sure you replace `SOME_ORDER_NUMBER` and `ZIP_CODE` with the relevant information.

Use the `--help` flag to display more details on usage:

     $ mft --help

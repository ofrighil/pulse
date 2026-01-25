use std::error::Error;

// use chrono::DateTime;
// use chrono_tz::{America::New_York, Tz};

use pulse_nyct::service::{Service, Services};
use pulse_nyct::train::{Direction, query_trains, arrivals_by_name};
// use pulse_parser::Stop;


fn main() -> Result<(), Box<dyn Error>> {
    let services = Services::from([Service::F]);

    // let trains = query_trains(services, Direction::North);
    // let stops = &trains[4];

    let stop_name = "34 St-Herald Sq";
    // let stop_id = "D17";

    // println!("{:#?}", stops);
    let res = arrivals_by_name(stop_name, services, Direction::North);

    println!("{:#?}", res);

    Ok(())
}

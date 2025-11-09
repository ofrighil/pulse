use std::error::Error;

use pulse_nyct::service::{Service, Services};
use pulse_nyct::train::{Direction, query_trains};

fn main() -> Result<(), Box<dyn Error>> {
    let services = Services::from([Service::F]);

    let results = query_trains(services, Direction::North);
    println!("{:#?}", results);

    Ok(())
}

use std::error::Error;

use clap::Parser;

use pulse_nyct::service::{Service, Services};
use pulse_nyct::train::{Direction, arrivals_by_name, filter_arrivals};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the stop
    #[arg(long)]
    stop_name: String,

    /// List of services
    #[arg(long)]
    service: Vec<String>,

    /// Going uptown (N) or downtown (S)
    #[arg(long)]
    direction: char,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let services: Services = args
        .service
        .iter()
        .map(|service| Service::from(service.as_str()))
        .collect::<Vec<Service>>()
        .into();

    let direction = Direction::from(args.direction);

    let arrivals = arrivals_by_name(&args.stop_name, services, direction);
    let arrivals = filter_arrivals(arrivals, 10) ;

    println!("{:#?}", arrivals);

    Ok(())
}

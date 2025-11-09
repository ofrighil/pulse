use chrono::DateTime;
use chrono_tz::{America::New_York, Tz};

use pulse_parser;
use pulse_parser::{Trip, parse_feed_message_bytes};

use super::service::{Service, Services};
use super::stops;

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
}

impl From<char> for Direction {
    fn from(direction: char) -> Direction {
        match direction {
            'N' => Direction::North,
            'S' => Direction::South,
            _ => panic!("Invalid direction"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Stop {
    id: String,
    pub name: String,
    // pub direction: Direction,
    pub arrival_time: DateTime<Tz>,
}

impl From<pulse_parser::Stop> for Stop {
    fn from(stop: pulse_parser::Stop) -> Stop {
        let id = stop.id[..=2].to_string();
        let arrival_time = DateTime::from_timestamp_secs(stop.arrival_time.unwrap())
            .unwrap()
            .with_timezone(&New_York);

        let name = match stops::STOPS.get(&id).copied() {
            Some(name) => name.to_string(),
            None => String::from(""), // Some stop IDs are not recorded in stop.txt
        };

        // let direction = stop.id.chars().nth_back(0).unwrap().into();

        Stop {
            id,
            name,
            // direction,
            arrival_time,
        }
    }
}

#[derive(Debug)]
pub struct Train {
    id: String,
    pub service: Service,
    pub direction: Direction,
    pub stops: Vec<Stop>,
}

impl From<Trip> for Train {
    fn from(trip: Trip) -> Train {
        // The (trip_)id has the format 021150_2..N08R. In this case, the
        // direction is the the 'N'.
        let direction: Direction = trip.id.split("..").collect::<Vec<&str>>()[1]
            .chars()
            .nth(0)
            .unwrap()
            .into();

        Train {
            id: trip.id.to_string(),
            service: trip.route_id.as_str().into(),
            direction,
            stops: trip.remaining_stops.into_iter().map(|s| s.into()).collect(),
        }
    }
}

fn request_trains(services: Services) -> Vec<Train> {
    let mut trains = Vec::new();

    for url in services.urls() {
        let buf = reqwest::blocking::get(url)
            .and_then(|r| r.bytes())
            .expect("request failed");
        let trips = parse_feed_message_bytes(&buf)
            .expect("parse failed")
            .into_iter()
            .filter(|trip| services.contains(&Service::from(trip.route_id.as_str())))
            .collect::<Vec<Trip>>();

        trains.extend(trips.into_iter().map(|trip| trip.into()));
    }

    trains
}

pub fn query_trains(services: Services, direction: Direction) -> Vec<Train> {
    request_trains(services)
        .into_iter()
        .filter(|train| train.direction == direction)
        .collect()
}

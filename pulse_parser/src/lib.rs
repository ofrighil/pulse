use std::error::Error;

use prost::Message;

use pulse_protos::transit_realtime::{FeedMessage, TripUpdate};
use pulse_protos::transit_realtime::trip_update::StopTimeUpdate;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Stop {
    id: String,
    arrival_time: i64,
    departure_time: i64,
}

impl From<StopTimeUpdate> for Stop {
    fn from(stop_time_update: StopTimeUpdate) -> Self {
        Stop {
            id: stop_time_update.stop_id.unwrap(),
            arrival_time: stop_time_update.arrival.unwrap().time.unwrap(),
            departure_time: stop_time_update.departure.unwrap().time.unwrap(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Trip {
    id: String,
    route_id: String,
    remaining_stops: Vec<Stop>,
}

impl From<TripUpdate> for Trip {
    fn from(trip_update: TripUpdate) -> Self {
        let trip = trip_update.trip;
        let remaining_stops: Vec<Stop> = trip_update.stop_time_update.into_iter().map(|s| s.into()).collect();
        Trip {
            id: trip.trip_id.unwrap(),
            route_id: trip.route_id.unwrap(),
            remaining_stops,
        }
    }
}

pub fn parse_feed_message_bytes(buf: &[u8]) -> Result<Vec<Trip>, Box<dyn Error>> {
    let feed_entities = if let Ok(feed) = FeedMessage::decode(buf) {
        feed.entity
    } else {
        panic!("Failed to decode input bytes");
    };

    let mut trips = Vec::new();

    for feed_entity in feed_entities.into_iter() {
        let trip_update = if let Some(trip_update) = feed_entity.trip_update {
            if trip_update.stop_time_update.is_empty() {
                continue;
            } else {
                trip_update
            }

        } else {
            continue;
        };

        // TODO: Alerts handled separately?
        let trip: Trip = trip_update.into();
        trips.push(trip);
    }

    Ok(trips)
}

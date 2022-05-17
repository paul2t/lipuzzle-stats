#![allow(unused_imports)]

extern crate csv;
extern crate serde;

use std::error::Error;
use std::io;
use serde::Deserialize;


#[derive(Deserialize)]
struct RatingAttempts {
    rating: u64,
    attemps: u64,
}

fn main() {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(io::stdin());
    let mut count = 0;
    let mut total_ratings : u64 = 0;
    let mut total_attemps : u64 = 0;
    let mut buckets_attempts = Vec::<u64>::new();
    let mut buckets_count = Vec::<u64>::new();
    let bucket_range = 100;

    for line in rdr.records() {
        let record: RatingAttempts = line.unwrap().deserialize(None).unwrap(); 
        total_ratings += record.rating;
        total_attemps += record.attemps;

        let bucket_index : usize = (record.rating / bucket_range) as usize;
        if buckets_attempts.len() <= bucket_index {
            buckets_attempts.resize(bucket_index + 1, 0);
            buckets_count.resize(bucket_index + 1, 0);
        }

        buckets_attempts[bucket_index] += record.attemps;
        buckets_count[bucket_index] += 1;

        if record.rating >= 2900 {
            println!("{} {} {}", count, record.rating, record.attemps);
        }

        if count < 10 {
            //println!("{} {} {}", count, record.rating, record.attemps);
        }
        count += 1;

        if count > 1000 {
            //break;
        }

    }

    println!("Rating: {} ({} avg)", total_ratings, (total_ratings as f64) / count as f64);
    println!("Attempts: {} ({} avg)", total_attemps, (total_attemps as f64) / count as f64);

    println!("Buckets");
    let mut bi = 0;
    let mut all_zeros = true;
    for b in buckets_attempts {
        let bc = buckets_count[bi];
        if !(all_zeros && bc == 0) {
            all_zeros = false;
            println!("{}, {}, {}", bi as u64 * bucket_range, bc, b);
        }
        bi += 1;
    }

}

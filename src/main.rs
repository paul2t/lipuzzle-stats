#![allow(unused_imports)]

extern crate serde;

use serde::Deserialize;
use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Deserialize, Default, Clone)]
struct PuzzleEntry {
    id: String,
    rating: u64,
    rating_deviation: u64,
    popularity: i64,
    attempts: u64,
}

#[derive(Clone, Default)]
struct Bucket {
    count: u64,
    rating: u64,
    rating_deviation: u64,
    attempts: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <FILEPATH>", args[0]);
        return;
    }

    let path = &args[1];
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut count = 0;
    let mut total_ratings: u64 = 0;
    let mut total_attempts: u64 = 0;
    let mut buckets = Vec::<Bucket>::new();
    let bucket_range = 100;
    let mut ten_highest_rated = Vec::<PuzzleEntry>::new();
    let mut highest_rated = PuzzleEntry::default();
    let mut lowest_rated = PuzzleEntry {
        rating: u64::MAX,
        ..Default::default()
    };
    let mut highest_count: u64 = 0;
    let mut lowest_count: u64 = 0;
    let mut skipped_count: u64 = 0;
    let mut least_popular = PuzzleEntry {
        popularity: i64::MAX,
        ..Default::default()
    };
    let mut most_played = PuzzleEntry::default();
    let mut least_played = PuzzleEntry {
        attempts: u64::MAX,
        ..Default::default()
    };

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let chunks: Vec<&str> = line.split(',').collect();
        let id = if !chunks.is_empty() {
            chunks[0].to_string()
        } else {
            String::new()
        };
        let rating = if chunks.len() > 3 {
            chunks[3].parse::<u64>().unwrap_or(0)
        } else {
            0
        };
        let rating_deviation = if chunks.len() > 4 {
            chunks[4].parse::<u64>().unwrap_or(0)
        } else {
            0
        };
        let popularity = if chunks.len() > 5 {
            chunks[5].parse::<i64>().unwrap_or(0)
        } else {
            0
        };
        let attempts = if chunks.len() > 6 {
            chunks[6]
                .parse::<i64>()
                .unwrap_or(0)
                .try_into()
                .unwrap_or(0)
        } else {
            0
        };
        if rating_deviation > 400 {
            skipped_count += 1;
            continue;
        }

        let record = PuzzleEntry {
            id,
            rating,
            rating_deviation,
            popularity,
            attempts,
        };

        total_ratings += record.rating;
        total_attempts += record.attempts;

        let bucket_index: usize = (record.rating / bucket_range) as usize;
        if buckets.len() <= bucket_index {
            buckets.resize(bucket_index + 1, Bucket::default());
        }

        let b: &mut Bucket = &mut buckets[bucket_index];
        b.count += 1;
        b.rating += record.rating;
        b.rating_deviation += record.rating_deviation;
        b.attempts += record.attempts;

        match record.rating.cmp(&highest_rated.rating) {
            Ordering::Greater => {
                highest_rated = record.clone();
                highest_count = 1
            }
            Ordering::Less => {}
            Ordering::Equal => highest_count += 1,
        };
        match record.rating.cmp(&lowest_rated.rating) {
            Ordering::Greater => {}
            Ordering::Less => {
                lowest_rated = record.clone();
                lowest_count = 1
            }
            Ordering::Equal => lowest_count += 1,
        };
        if least_popular.popularity > record.popularity {
            least_popular = record.clone();
        }
        if most_played.attempts < record.attempts {
            most_played = record.clone();
        }
        if least_played.attempts > record.attempts {
            least_played = record.clone();
        }

        let mut insert_index: Option<usize> = None;
        for (pi, p) in ten_highest_rated.iter().enumerate() {
            if p.rating < rating {
                insert_index = Some(pi);
                break;
            }
        }
        if let Some(pi) = insert_index {
            if ten_highest_rated.len() > 10 {
                ten_highest_rated.pop();
            }
            ten_highest_rated.insert(pi, record.clone());
        } else if ten_highest_rated.len() < 10 {
            ten_highest_rated.push(record.clone());
        }

        count += 1;
    }

    println!(
        "Count: {} (+{} = {})",
        count,
        skipped_count,
        count + skipped_count
    );
    println!(
        "Rating: {} ({} avg)",
        total_ratings,
        (total_ratings as f64) / count as f64
    );
    println!(
        "Attempts: {} ({} avg)",
        total_attempts,
        (total_attempts as f64) / count as f64
    );
    println!(
        "Highest rated: {} {} (x{}) {} {}",
        highest_rated.id,
        highest_rated.rating,
        highest_count,
        highest_rated.attempts,
        highest_rated.rating_deviation
    );
    println!(
        "Lowest rated: {} {} (x{}) {} {}",
        lowest_rated.id,
        lowest_rated.rating,
        lowest_count,
        lowest_rated.attempts,
        lowest_rated.rating_deviation
    );
    println!(
        "Most played: {} {} {}",
        most_played.id, most_played.rating, most_played.attempts
    );
    println!(
        "Least played: {} {} {}",
        least_played.id, least_played.rating, least_played.attempts
    );
    println!("10 Highest rated:");
    for p in ten_highest_rated {
        println!(
            "{} / rating: {} / attempts: {} / deviation: {}",
            p.id, p.rating, p.attempts, p.rating_deviation
        );
    }

    println!("Buckets");
    let mut all_zeros = true;
    println!("rating, count, attempts, deviation, attempts/puzzle, deviation/puzzle");
    for (bi, b) in buckets.iter().enumerate() {
        if all_zeros && b.count == 0 {
            continue;
        }
        all_zeros = false;
        println!(
            "{}, {}, {}, {}, {}, {}",
            bi as u64 * bucket_range,
            b.count,
            b.attempts,
            b.rating_deviation,
            b.attempts as f64 / b.count as f64,
            b.rating_deviation as f64 / b.count as f64
        );
    }
}

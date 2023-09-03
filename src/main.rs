#![allow(unused_imports)]

extern crate serde;

use serde::Deserialize;
use std::env;
use std::error::Error;
use std::io;

#[derive(Deserialize, Default, Clone)]
struct PuzzleEntry {
    id: String,
    _fen: String,
    _moves: String,
    rating: u64,
    rating_deviation: u64,
    popularity: i64,
    attempts: u64,
    _themes: String,
    _url: String,
    _opening_family: String,
    _opening_variation: String,
}

#[derive(Clone, Default)]
struct Bucket {
    count: u64,
    rating: u64,
    rating_deviation: u64,
    attempts: u64,
}

fn main() {
    let mut count = 0;
    let mut total_ratings: u64 = 0;
    let mut total_attempts: u64 = 0;
    let mut buckets = Vec::<Bucket>::new();
    let bucket_range = 100;
    let mut highest_rated: PuzzleEntry = PuzzleEntry::default();
    let mut lowest_rated = PuzzleEntry {
        rating: u64::MAX,
        ..Default::default()
    };
    let mut least_popular = PuzzleEntry {
        popularity: i64::MAX,
        ..Default::default()
    };
    let mut most_played = PuzzleEntry::default();
    let mut least_played = PuzzleEntry {
        attempts: u64::MAX,
        ..Default::default()
    };

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let chunks: Vec<&str> = line.split(',').collect();
        let id = if !chunks.is_empty() {
            chunks[0].to_string()
        } else {
            String::new()
        };
        let _fen = if chunks.len() > 1 {
            chunks[1].to_string()
        } else {
            String::new()
        };
        let _moves = if chunks.len() > 2 {
            chunks[2].to_string()
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
        let _themes = if chunks.len() > 7 {
            chunks[7].to_string()
        } else {
            String::new()
        };
        let _url = if chunks.len() > 8 {
            chunks[8].to_string()
        } else {
            String::new()
        };
        let _opening_family = if chunks.len() > 9 {
            chunks[9].to_string()
        } else {
            String::new()
        };
        let _opening_variation = if chunks.len() > 10 {
            chunks[10].to_string()
        } else {
            String::new()
        };

        let record = PuzzleEntry {
            id,
            _fen,
            _moves,
            rating,
            rating_deviation,
            popularity,
            attempts,
            _themes,
            _url,
            _opening_family,
            _opening_variation,
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

        if highest_rated.rating < record.rating {
            highest_rated = record.clone();
        }
        if lowest_rated.rating > record.rating {
            lowest_rated = record.clone();
        }
        if least_popular.popularity > record.popularity {
            least_popular = record.clone();
        }
        if most_played.attempts < record.attempts {
            most_played = record.clone();
        }
        if least_played.attempts > record.attempts {
            least_played = record.clone();
        }

        if record.rating >= 2900 || record.rating < 550 {
            println!(
                "[{}] {} / rating: {} / attempts: {} / deviation: {}",
                count, record.id, record.rating, record.attempts, record.rating_deviation
            );
        }

        count += 1;
    }

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
        "Highest rated: {} {} {}",
        highest_rated.id, highest_rated.rating, highest_rated.attempts
    );
    println!(
        "Lowest rated: {} {} {}",
        lowest_rated.id, lowest_rated.rating, lowest_rated.attempts
    );
    println!(
        "Most played: {} {} {}",
        most_played.id, most_played.rating, most_played.attempts
    );
    println!(
        "Least played: {} {} {}",
        least_played.id, least_played.rating, least_played.attempts
    );

    println!("Buckets");
    let mut all_zeros = true;
    println!("rating, count, attempts, deviation, attempts/puzzle, deviation/puzzle");
    for (bi, b) in buckets.into_iter().enumerate() {
        if !(all_zeros && b.count == 0) {
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
}

use rayon::prelude::*;
use std::{io, time::Instant};
use strsim::{
    damerau_levenshtein, hamming, jaro, jaro_winkler, levenshtein, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance, sorensen_dice,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional file to operate on
    input_file: Option<String>,

    /// Optional field index to operate on
    index: Option<i8>,

    #[arg(
        short,
        long,
        default_value = "jaro_winkler",
        help = "text metric to use. 
    available: hamming, levenshtein, normalized_levenshtein, osa_distance,
             damerau_levenshtein, normalized_damerau_levenshtein, jaro,
             jaro_winkler, sorensen_dice"
    )]
    metric: String,

    #[arg(
        short,
        long,
        default_value = "0",
        help = "metric threshold, only return matches above this value"
    )]
    threshold: f64,

    #[arg(long, default_value = "false", help = "debug output")]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();
    let records: Vec<csv::StringRecord> = if let Some(input) = cli.input_file {
        csv::Reader::from_path(input)
            .unwrap()
            .records()
            .filter_map(|r| r.ok())
            .collect()
    } else {
        csv::Reader::from_reader(io::stdin())
            .records()
            .filter_map(|r| r.ok())
            .collect()
    };

    let idx = cli.index.unwrap_or(0) as usize;
    let target: Vec<_> = records.iter().map(|r| r[idx].to_string()).collect();
    let len = target.len();

    let thres = cli.threshold;

    let now = Instant::now();
    let rs = target
        .clone()
        .into_par_iter()
        .enumerate()
        .map(|(i, x)| {
            if i == len - 1 {
                return None;
            }

            let mut v = Vec::new();
            for j in (i + 1)..len {
                let left = x.as_str();
                let right = target[j].as_str();
                let m = match cli.metric.as_str() {
                    "hamming" => hamming(left, right).unwrap_or(0) as f64,
                    "levenshtein" => levenshtein(left, right) as f64,
                    "normalized_levenshtein" => normalized_levenshtein(left, right) as f64,
                    "osa_distance" => osa_distance(left, right) as f64,
                    "damerau_levenshtein" => damerau_levenshtein(left, right) as f64,
                    "normalized_damerau_levenshtein" => {
                        normalized_damerau_levenshtein(left, right) as f64
                    }
                    "jaro" => jaro(left, right) as f64,
                    "jaro_winkler" => jaro_winkler(left, right) as f64,
                    "sorensen_dice" => sorensen_dice(left, right) as f64,
                    _ => 0.0,
                };

                if m - thres > 0.0001 {
                    if cli.debug {
                        println!("{:?}", (i, left, j, right, m));
                    }
                    v.push((i, j, m));
                }
            }

            if v.len() == 0 {
                return None;
            }

            Some(v)
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .flatten()
        .collect::<Vec<(usize, usize, f64)>>();

    if !cli.debug {
        rs.iter().for_each(|(i, j, m)| {
            println!("{},{},{:.2}", i, j, m);
        });
    }

    if cli.debug {
        eprintln!("time: {:?}", now.elapsed());
    }
}

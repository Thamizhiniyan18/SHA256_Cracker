use clap::Parser;
use regex::Regex;
use std::io::BufRead;
use std::path::PathBuf;
use std::fs;
use std::process::exit;
use std::time::{Duration, Instant};
use sha2::{Sha256, Digest};

/// SHA256 Hash Cracker
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// SHA256 hash to be cracked
    #[arg(short, long)]
    sha256: String,

    /// Path to the wordlist to be used
    #[arg(short, long)]
    wordlist: PathBuf,
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let args = Args::parse();

    // Regex pattern to verify whether the given hash is SHA256
    let regex_sha256: Regex = match Regex::new(r"\b[a-zA-Z0-9]{64}\b") {
        Err(why) => panic!("Couldn't parse the given regex pattern: {why}"),
        Ok(regex_sha265) => regex_sha265
    };

    // Verifying whether the given hash is a valid SHA256 hash
    match regex_sha256.is_match(&args.sha256) {
        true => println!("[+] Given Hash: {}", args.sha256),
        false => panic!("The given Hash is not a valid SHA256 hash")
    };

    // Checking whether the given wordlist path is a valid path
    match args.wordlist.exists() {
        true => {
            // Checking whether the given wordlist path is a valid file
            match args.wordlist.is_file() {
                true => println!("[+] Given Wordlist: {}", args.wordlist.display()),
                false => println!("The wordlist path is not a valid file: {}", args.wordlist.display())
            }
        },
        false => println!("The given path is not valid: {}", args.wordlist.display())
    }

    // Initializing a Vector for storing the wordlist
    let mut wordlist_content = Vec::new();

    println!("Reading/Loading the wordlist to RAM: {}", args.wordlist.display());
    let wordlist_read_start_time: Instant = Instant::now();
    
    // Reading/Loading the wordlist to RAM
    match fs::read(&args.wordlist) {
        Err(why) => panic!("Couldn't read the given wordlist {}: {}", args.wordlist.display(), why),
        Ok(file_content) => {
            for word in file_content.lines() {
                match word {
                    Err(_) => continue,
                    Ok(w) => wordlist_content.push(w),
                }
            }
        }
    }

    let wordlist_read_time_taken: Duration = Instant::elapsed(&wordlist_read_start_time);
    println!("Successfully Read/Loaded the wordlist to RAM: {}", args.wordlist.display());
    println!("Time taken to Reading/Loading the wordlist to RAM: {} seconds", wordlist_read_time_taken.as_secs_f64());
    println!("Total number of valid utf-8 words from the given wordlist : {}", wordlist_content.len());

    println!("Starting to crack the given hash !!!");
    let crack_start_time: Instant = Instant::now();

    for word in wordlist_content {
        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(&word);

        // read hash digest and consume hasher
        let generated_hash = format!("{:x}", hasher.finalize());

        if generated_hash == args.sha256 {
            println!("[+] Successfully cracked the given hash => {} : {}", args.sha256, &word);

            let crack_time_taken: Duration = Instant::elapsed(&crack_start_time);
            println!("Time taken to crack the given hash: {} seconds", crack_time_taken.as_secs_f64());

            let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);
            println!("Total Time taken: {} seconds", total_time_taken.as_secs_f64());

            exit(0);
        }
    }

    println!("The value for the given hash is not found in the given wordlist.");
    println!("BETTER LUCK NEXT TIME !!!!!");

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);
    println!("Total Time taken: {} seconds", total_time_taken.as_secs_f64());
}
use reqwest::blocking::get;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;
use std::env;
use std::error::Error;

fn download_avatar(id: u32, folder: &str) -> Result<(), Box<dyn Error>> {
    // URL for random avatar generation
    let url = "https://www.tapback.co/api/avatar.webp";
    
    // Send a GET request to download the avatar
    let response = get(url)?;
    
    // Create the file path in the specified folder with unique filenames
    let file_path = format!("{}/avatar_{}.webp", folder, id);
    let mut file = File::create(&file_path)?;
    
    // Write the downloaded content into the file
    copy(&mut response.bytes()?.as_ref(), &mut file)?;
    println!("Downloaded avatar to: {}", file_path);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <number_of_avatars> <output_folder>", args[0]);
        std::process::exit(1);
    }

    // Parse the number of avatars and output folder
    let num_avatars: u32 = args[1].parse()?;
    let folder = &args[2];

    // Create the output directory if it doesn’t exist
    create_dir_all(folder)?;

    // Loop to download the specified number of avatars
    for i in 1..=num_avatars {
        if let Err(e) = download_avatar(i, folder) {
            eprintln!("Failed to download avatar {}: {}", i, e);
        }
    }

    Ok(())
}
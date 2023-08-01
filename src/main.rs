use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use shell_words::quote;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <filename> <number>", args[0]);
        return;
    }

    // Read the file specified in the command line arguments
    let filename = &args[1];

    // Check if the file exists
    let file_path = Path::new(filename);
    if !file_path.exists() {
        println!("Error: The file '{}' does not exist.", filename);
        return;
    }

    // Parse the number from the command line arguments
    let number: i32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: Invalid number provided.");
            return;
        }
    };

    // Call the general cut function based on the file type
    if filename.to_lowercase().ends_with(".mp4") {
        if let Err(error) = cut(filename, number, cut_video) {
            println!("Error: {}", error);
        } else {
            println!("Video successfully cut and saved as '{}_out.mp4'", filename);
        }
    } else if filename.to_lowercase().ends_with(".mp3") {
        if let Err(error) = cut(filename, number, cut_mp3) {
            println!("Error: {}", error);
        } else {
            println!("Audio successfully cut and saved as '{}_out.mp3'", filename);
        }
    } else {
        println!("Error: Unsupported file format. Supported formats: mp4, mp3");
    }
}

fn cut_video(filename: &str, number: i32) -> Result<(), String> {
    // Generate the output filename with the "_out" suffix
    let output_filename = format!("{}_out.mp4", filename);

    // Prepare the FFmpeg command to cut the video
    let ffmpeg_cmd = format!(
        "ffmpeg -i {} -ss {} -t 30 -c:v copy -c:a copy {}",
        filename, number, output_filename
    );

    // Run the FFmpeg command as an external process
    let status = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_cmd)
        .status()
        .map_err(|_| "Failed to execute FFmpeg command.")?;

    // Call the function to convert the video to audio (MP3) and delete the original video
    if let Err(error) = convert_to_audio_and_delete(filename) {
        println!("Error: {}", error);
    } else {
        println!(
            "Video successfully converted to audio (MP3) of 30 secs",
        );
    }

    // Check if the FFmpeg command was successful
    if status.success() {
        Ok(())
    } else {
        Err("FFmpeg command failed.".to_string())
    }


}

fn convert_to_audio_and_delete(filename: &str) -> Result<(), String> {
    // Generate the input and output filenames
    let input_filename = format!("{}_out.mp4", filename);
    let mut output_filename = format!("{}.mp3", filename.trim_end_matches(".mp4"));

    // Check if the output filename already exists, if yes, add a number to make it unique
    let mut number = 1;
    while std::path::Path::new(&output_filename).exists() {
        output_filename = format!("{}_{}.mp3", filename.trim_end_matches(".mp4"), number);
        number += 1;
    }

    // Prepare the FFmpeg command to convert video to audio
    let ffmpeg_cmd = format!(
        "ffmpeg -i {} -vn -c:a libmp3lame {}",
        input_filename, output_filename
    );

    // Run the FFmpeg command to convert video to audio
    let status = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_cmd)
        .status()
        .map_err(|_| "Failed to execute FFmpeg command for audio conversion.")?;

    // Check if the FFmpeg command was successful
    if !status.success() {
        return Err("FFmpeg command for audio conversion failed.".to_string());
    }

    // Remove the original video file
    if let Err(error) = std::fs::remove_file(&input_filename) {
        return Err(format!(
            "Failed to delete the original video file '{}': {}",
            input_filename, error
        ));
    }
    println!("Mp3 file {} successfully created", output_filename);

    Ok(())
}

fn cut_mp3(filename: &str, number: i32) -> Result<(), String> {
    let mut output_filename = format!("{}", filename);

    // Check if the output filename already exists, if yes, add a number to make it unique
    let mut number = 1;
    while std::path::Path::new(&output_filename).exists() {
        output_filename = format!("{}_{}.mp3", filename.trim_end_matches(".mp3"), number);
        number += 1;
    }

    // Escape the filenames for use in the FFmpeg command
    let escaped_input_filename = quote(filename);
    let escaped_output_filename = quote(&output_filename);

    // Prepare the FFmpeg command to cut the mp3
    let ffmpeg_cmd = format!(
        "ffmpeg -i {} -ss {} -t 30 -c:v copy -c:a copy {}",
        escaped_input_filename, number, escaped_output_filename
    );

    println!("FFmpeg command {}", ffmpeg_cmd);

    // Run the FFmpeg command as an external process
    let status = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_cmd)
        .status()
        .map_err(|_| "Failed to execute FFmpeg command.")?;

    // Check if the FFmpeg command was successful
    if status.success() {
        Ok(())
    } else {
        Err("FFmpeg command failed.".to_string())
    }
}

fn cut<F>(filename: &str, number: i32, cut_function: F) -> Result<(), String>
where
    F: Fn(&str, i32) -> Result<(), String>,
{
    cut_function(filename, number)
}

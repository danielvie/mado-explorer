use std::path::PathBuf;

use mado_image_preview::{DecodeLimits, decode};

fn main() -> std::process::ExitCode {
    let Some(path) = std::env::args_os().nth(1) else {
        println!("Usage: task run -- <image-path>");
        println!("CPU decoder probe only. Codec working-memory limits are not hard-enforced.");
        return std::process::ExitCode::SUCCESS;
    };
    let limits = DecodeLimits {
        max_input_bytes: 16 * 1024 * 1024,
        max_width: 8192,
        max_height: 8192,
        max_output_bytes: 16 * 1024 * 1024,
        max_buffer_bytes: 64 * 1024 * 1024,
        max_decoder_bytes: 64 * 1024 * 1024,
    };
    match decode(&PathBuf::from(path), limits) {
        Ok(preview) => {
            println!(
                "{:?}: {} x {}, {} RGBA bytes, {} file bytes, applied orientation {}",
                preview.metadata.format,
                preview.width,
                preview.height,
                preview.rgba.len(),
                preview.metadata.file_bytes,
                preview.metadata.orientation
            );
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}

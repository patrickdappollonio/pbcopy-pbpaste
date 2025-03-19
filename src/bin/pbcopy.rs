use anyhow::{Context, Result};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{self, Read};

fn main() -> Result<()> {
    // Initialize the clipboard context, converting the error into anyhow::Error.
    let mut clipboard = ClipboardContext::new()
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to initialize the clipboard. Is it supported on your system?")?;

    // Read from standard input.
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from standard input. Please ensure you're piping valid text.")?;

    // Copy the content into the clipboard.
    clipboard
        .set_contents(buffer)
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to copy text to the clipboard. Please try again.")?;

    Ok(())
}

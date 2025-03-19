use anyhow::{Context, Result};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{self, Write};

fn main() -> Result<()> {
    // Initialize the clipboard context, converting the error into anyhow::Error.
    let mut clipboard = ClipboardContext::new()
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to initialize the clipboard. Is it supported on your system?")?;

    // Retrieve the text from the clipboard.
    let content = clipboard
        .get_contents()
        .map_err(|e| anyhow::anyhow!(e))
        .context(
            "Failed to retrieve text from the clipboard. Please ensure there is text copied.",
        )?;

    // Write the content to standard output.
    io::stdout()
        .write_all(content.as_bytes())
        .context("Failed to write the text to standard output. Please try again.")?;

    Ok(())
}

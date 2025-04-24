use std::future::Future;

use anyhow::Result;
use tokio::spawn;

pub fn spawn_and_forget<F, O>(future: F)
where
    F: Future<Output = Result<O>> + Send + 'static,
{
    spawn(async move {
        if let Err(err) = future.await {
            print_error_details(&err);
        }
    });
}

use anyhow::Error;

fn print_error_details(err: &Error) {
    let mut output = String::new();

    output.push_str(&"-".repeat(80));
    output.push_str(&format!("\nError: {err}\n"));
    output.push_str("\nCaused by:\n");

    for (i, cause) in err.chain().enumerate() {
        output.push_str(&format!("  {i}: {cause}\n"));
    }

    output.push_str("\nBacktrace:\n");
    output.push_str(&format!("{}\n", err.backtrace()));
    output.push_str(&"-".repeat(80));

    eprintln!("{output}\n");
}

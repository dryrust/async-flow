// This is free and unencumbered software released into the public domain.

use async_flow::{
    io::Result,
    tokio::{Input, Output, System},
};

/// cargo run --example sqrt
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (input, sqrt_in) = async_flow::tokio::bounded(1);
    let (sqrt_out, output) = async_flow::tokio::bounded(1);

    let mut system = System::new();
    let _ = system.stdin::<f64>(input);
    system.spawn(sqrt(sqrt_in, sqrt_out));
    let _ = system.stdout::<f64>(output);

    system.execute().await
}

async fn sqrt(mut input: Input<f64>, output: Output<f64>) -> Result {
    while let Some(value) = input.recv().await? {
        output.send(value.sqrt()).await?;
    }
    Ok(())
}

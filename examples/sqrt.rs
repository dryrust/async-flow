// This is free and unencumbered software released into the public domain.

use async_flow::{
    io::Result,
    tokio::{Input, Output},
};

/// cargo run --example sqrt
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (input, sqrt_in) = async_flow::tokio::bounded(1);
    let (sqrt_out, mut output) = async_flow::tokio::bounded(1);

    tokio::spawn(async move {
        input.send(42.0).await.unwrap();
    });

    tokio::spawn(sqrt(sqrt_in, sqrt_out));

    while let Some(result) = output.recv().await {
        println!("{}", result);
    }
}

async fn sqrt(mut input: Input<f64>, output: Output<f64>) -> Result<(), f64> {
    while let Some(value) = input.recv().await {
        output.send(value.sqrt()).await?;
    }
    Ok(())
}

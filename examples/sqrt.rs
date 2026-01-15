// This is free and unencumbered software released into the public domain.

use async_flow::{
    io::Result,
    tokio::{Inputs, Outputs, System},
};

/// cargo run --example sqrt
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut system = System::new();
    let sqrt_in = system.stdin::<f64>();
    let sqrt_out = system.stdout::<f64>();
    system.spawn(sqrt(sqrt_in, sqrt_out));
    system.execute().await
}

async fn sqrt(mut inputs: Inputs<f64>, outputs: Outputs<f64>) -> Result {
    while let Some(input) = inputs.recv().await? {
        let output = input.sqrt();
        outputs.send(output).await?;
    }
    Ok(())
}

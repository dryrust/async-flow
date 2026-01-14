// This is free and unencumbered software released into the public domain.

/// cargo run --example basic
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (output, mut input) = async_flow::tokio::bounded_boxed(1);

    tokio::spawn(async move {
        output.send("value1").await.unwrap();
        output.send("value2").await.unwrap();
    });

    while let Some(message) = input.recv().await.unwrap() {
        eprintln!("recv: {}", message);
    }
}

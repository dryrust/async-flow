// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;

pub const DYNAMIC: usize = 0;

pub fn single<T>() -> (Output<T>, Input<T>) {
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    let outputs = Output::<T>::from(tx);
    let inputs = Input::<T>::from(rx);
    (outputs, inputs)
}

pub fn bounded<T>(buffer: usize) -> (Outputs<T, DYNAMIC>, Inputs<T, DYNAMIC>) {
    let (tx, rx) = tokio::sync::mpsc::channel(buffer);
    let outputs = Outputs::<T, DYNAMIC>::from(tx);
    let inputs = Inputs::<T, DYNAMIC>::from(rx);
    (outputs, inputs)
}

pub fn bounded_boxed<T>(
    buffer: usize,
) -> (
    Box<dyn crate::io::OutputPort<T> + Send>,
    Box<dyn crate::io::InputPort<T> + Send>,
)
where
    T: Send + Sync + 'static,
{
    let (outputs, inputs) = bounded::<T>(buffer);
    (Box::new(outputs), Box::new(inputs))
}

mod connection;
//pub use connection::*;

mod input;
pub use input::*;

mod inputs;
pub use inputs::*;

mod output;
pub use output::*;

mod outputs;
pub use outputs::*;

#[cfg(feature = "std")]
mod stderr;
#[cfg(feature = "std")]
pub use stderr::*;

#[cfg(feature = "std")]
mod stdin;
#[cfg(feature = "std")]
pub use stdin::*;

#[cfg(feature = "std")]
mod stdout;
#[cfg(feature = "std")]
pub use stdout::*;

mod system;
pub use system::*;

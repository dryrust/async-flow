// This is free and unencumbered software released into the public domain.

//! This crate provides async flow traits.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

mod input_port;
pub use input_port::*;

mod output_port;
pub use output_port::*;

pub fn bounded<T>(buffer: usize) -> (OutputPort<T>, InputPort<T>) {
    let (tx, rx) = tokio::sync::mpsc::channel(buffer);
    let output = OutputPort::from(tx);
    let input = InputPort::from(rx);
    (output, input)
}

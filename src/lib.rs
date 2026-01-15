// This is free and unencumbered software released into the public domain.

//! This crate provides async flow traits.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod io;

#[cfg(feature = "flume")]
pub mod flume;

#[cfg(feature = "std")]
pub mod stdio;

#[cfg(feature = "tokio")]
pub mod tokio;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

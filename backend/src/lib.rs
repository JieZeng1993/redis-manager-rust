#![feature(str_split_as_str)]

pub mod service;
pub mod domain;
pub mod mapper;
pub mod config;
pub mod mix;
pub mod rest;
pub mod util;
pub mod enums;

#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

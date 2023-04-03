use clap::{Parser, Subcommand};
use std::net::IpAddr;

#[derive(Debug, Parser)]
struct Args {
    ip_addr: Option<IpAddr>,
}

use chrono::Local;
use clap::Parser;
use console::Emoji;
use dotenv::dotenv;
use log::{debug, error, info, trace, Level, LevelFilter};
use std::{
    io::Write,
    time::{Duration, SystemTime},
};
use tokio::time;

use crate::util::is_ip;
use util::{debug_style, error_style, info_style, success_style, warn_style};

mod dns_provider;
mod ip_provider;
mod util;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Options {
    #[clap(
        short = 'd',
        long = "dns",
        required = true,
        help = "The DNS who providing resolving of your host."
    )]
    dns: String,
    #[clap(
        short = 'i',
        long = "ip-provider",
        required = true,
        help = "The provider who detecting and providing your public IP."
    )]
    ip_provider: String,
    #[clap(
        long = "interval",
        default_value = "60",
        help = "The interval to request IP provider for public IP."
    )]
    interval: u64,
    #[clap(long = "domain", required = true, help = "The domain of your DNS record.")]
    domain: String,
    #[clap(long = "record-type", default_value = "A", help = "The type of your DNS record.")]
    record_type: String,
    #[clap(
        long = "record-host",
        help = "The host of your DNS record, just like what your config on DNS."
    )]
    record_host: Option<String>,
    #[clap(long = "record-ttl", default_value = "300", help = "The ttl of your DNS record.")]
    record_ttl: u32,
    #[clap(
        short = 'v',
        long = "verbose",
        parse(from_occurrences),
        help = "The level of log verbosity."
    )]
    verbose: u32,
}

#[tokio::main]
async fn main() {
    // Parse options
    let options: Options = Options::parse();
    // println!("{:?}", options);

    dotenv().ok();
    init_log(&options);

    let mut last_updated_ip: Option<String> = None;
    let mut last_updated_at: Option<SystemTime> = None;
    let update_period = Duration::from_secs(options.record_ttl.into());

    info!("DDNS with DNS has started {}", Emoji("✨", ""));
    debug!("DNS: {}", info_style(&options.dns));
    debug!(
        "Will request public IP from [{}] every [{}] seconds.",
        info_style(&options.ip_provider),
        info_style(&options.interval)
    );

    let mut timer = time::interval(Duration::from_secs(options.interval));
    loop {
        timer.tick().await;

        let started_at = SystemTime::now();
        let ret = ip_provider::get_ip(&options.ip_provider).await;
        let duration = SystemTime::now()
            .duration_since(started_at)
            .expect("Clock may have gone backwards");

        if ret.is_err() {
            continue;
        }

        let ip = ret.unwrap();

        if !is_ip(&ip) {
            error!(target: "error", "Got an invalid IP address!");
            continue;
        } else {
            info!(
                target: "success",
                "Successfully got current public IP: {} (in {}ms)",
                success_style(&ip),
                info_style(duration.as_millis())
            );
        }

        // If the last IP update is the same as the current IP and the update cycle has not yet been reached,
        // then skip.
        if last_updated_ip.is_some() && last_updated_at.is_some() {
            let since_last_updated = SystemTime::now()
                .duration_since(last_updated_at.to_owned().unwrap())
                .expect("Clock may have gone backwards");
            if last_updated_ip.as_ref().unwrap() == &ip && since_last_updated < update_period {
                info!("No need to update, skip.");
                continue;
            }
        }

        let started_at = SystemTime::now();
        let ret = dns_provider::update_record(
            &options.dns,
            &options.domain,
            &options.record_type,
            options.record_host.as_deref(),
            &ip,
            &options.record_ttl,
        )
        .await;
        let duration = SystemTime::now()
            .duration_since(started_at)
            .expect("Clock may have gone backwards");

        if ret.is_err() {
            continue;
        }

        // Save IP and SystemTime when DNS update succeeds.
        last_updated_ip = Some(ip);
        last_updated_at = Some(SystemTime::now());

        info!(
            target: "success",
            "Successfully updated dns record! (in {}ms)",
            info_style(duration.as_millis())
        );
    }
}

fn init_log(options: &Options) {
    let level = match options.verbose {
        2 => LevelFilter::Trace,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Info,
    };

    let mut builder = env_logger::Builder::new();
    builder
        .filter(Some("dwd"), level)
        .filter(Some("error"), level)
        .filter(Some("success"), level)
        .format(|buf, record| {
            let mut char = match record.target() {
                "success" => success_style("✔ "),
                "error" => error_style("✗ "),
                _ => success_style(""),
            };
            if record.level() == Level::Error {
                char = error_style("✗ ");
            }

            let level = match record.level() {
                Level::Error => error_style(record.level()),
                Level::Warn => warn_style(record.level()),
                Level::Info => info_style(record.level()),
                _ => debug_style(record.level()),
            };

            writeln!(
                buf,
                "[{}] [{:<5}] {}{}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                char,
                &record.args(),
            )
        })
        .init();

    match options.verbose {
        2 => trace!("{}{}", Emoji("📃 ", ""), error_style("Log verbosity level: trace")),
        1 => debug!("{}{}", Emoji("📃 ", ""), warn_style("Log verbosity level: debug")),
        0 | _ => (),
    }
}

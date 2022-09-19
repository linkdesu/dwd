use chrono::Local;
use clap::Parser;
use console::Emoji;
use dotenv::dotenv;
use log::{debug, error, info, trace, Level, LevelFilter};
use std::{
    io::Write,
    process,
    time::{Duration, SystemTime},
};
use tokio::{task, time};

use util::{debug_style, error_style, info_style, success_style, warn_style};

mod config;
mod dns_provider;
mod ip_provider;
mod util;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Options {
    #[clap(
        short = 'c',
        long = "config",
        required = true,
        help = "Use a config file to configure behaviors intead of the command line options."
    )]
    config: String,
    #[clap(
        short = 'v',
        long = "verbose",
        parse(from_occurrences),
        help = "The level of log verbosity."
    )]
    verbose: u32,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Parse options
    let options: Options = Options::parse();
    // println!("{:?}", options);

    dotenv().ok();
    init_log(&options);

    let conf = match config::load_config(&options.config) {
        Ok(val) => val,
        Err(err) => {
            error!(target: "error", "{}", err);
            process::exit(1);
        }
    };

    info!("DDNS with DNS has started {}", Emoji("✨", ""));
    debug!(
        "Will request public IP from [{}] every {} seconds and update to [{}].",
        info_style(&conf.ip_provider.join(", ")),
        info_style(&conf.interval),
        info_style(&conf.dns_provider.join(", "))
    );

    let handle = task::spawn(async move {
        let mut last_updated_ip: Option<String> = None;
        let mut last_updated_at: Option<SystemTime> = None;

        let mut timer = time::interval(Duration::from_secs(conf.interval as u64));
        loop {
            timer.tick().await;

            let started_at = SystemTime::now();
            let ret = ip_provider::get_ip_by_fallback(&conf.ip_provider).await;
            let duration = SystemTime::now()
                .duration_since(started_at)
                .expect("Clock may have gone backwards");

            let (provider_name, ip) = match ret {
                None => continue,
                Some(val) => val,
            };

            if !util::is_ip(&ip) {
                error!(target: "error", "Got an invalid IP address!");
                continue;
            } else {
                info!(
                    target: "success",
                    "[{}] Successfully got current public IP: {} (in {}ms)",
                    provider_name,
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
                if last_updated_ip.as_ref().unwrap() == &ip {
                    info!(
                        "No need to update the DNS record, skip.(since_last_updated: {}s)",
                        since_last_updated.as_secs()
                    );
                    continue;
                }
            }

            let started_at = SystemTime::now();
            dns_provider::update_dns_for_all(&conf, &ip).await;
            let duration = SystemTime::now()
                .duration_since(started_at)
                .expect("Clock may have gone backwards");

            // Save IP and SystemTime when DNS update succeeds.
            last_updated_ip = Some(ip);
            last_updated_at = Some(SystemTime::now());

            info!(
                "The DNS record updated in {}ms {}",
                info_style(duration.as_millis()),
                Emoji("🕐", "")
            );
        }
    });
    handle.await.expect("DWD exits unexpectedly, sorry for that. 💔");
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

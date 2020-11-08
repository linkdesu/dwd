use clap::Clap;
use console::Emoji;
use tokio::time;
use std::time::{Duration, SystemTime};
use tokio::time;

use util::{error, info, set_verbosity_level, success, v0, v1, v2, verbose_log, warn};

mod dns_provider;
mod ip_provider;
mod util;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Link Xie. <xieaolin@gmail.com>")]
struct Options {
    #[clap(
        short = 'd',
        long = "dns",
        required = true,
        about = "The DNS who providing resolving of your host."
    )]
    dns: String,
    #[clap(
        short = 'i',
        long = "ip-provider",
        required = true,
        about = "The provider who detecting and providing your public IP."
    )]
    ip_provider: String,
    #[clap(
        short = 'I',
        long = "interval",
        default_value = "60",
        about = "The interval to request IP provider for public IP."
    )]
    interval: u64,
    #[clap(
        short = 't',
        long = "domain",
        required = true,
        about = "The domain of your DNS record."
    )]
    domain: String,
    #[clap(
        short = 't',
        long = "record-type",
        default_value = "A",
        about = "The type of your DNS record."
    )]
    record_type: String,
    #[clap(
        short = 'h',
        long = "record-host",
        about = "The host of your DNS record, just like what your config on DNS."
    )]
    record_host: Option<String>,
    #[clap(
        short = 'T',
        long = "record-ttl",
        default_value = "300",
        about = "The ttl of your DNS record."
    )]
    record_ttl: u32,
    #[clap(
        short = 'v',
        long = "verbose",
        parse(from_occurrences),
        about = "The level of log verbosity."
    )]
    verbose: u32,
}

#[tokio::main]
async fn main() {
    // Parse options
    let options: Options = Options::parse();
    // println!("{:?}", options);

    set_verbosity_level(options.verbose as usize);
    match options.verbose {
        2 => v2!(
            "{}{}",
            Emoji("📃 ", ""),
            error("Log verbosity level: crazy")
        ),
        1 => v1!(
            "{}{}",
            Emoji("📃 ", ""),
            warn("Log verbosity level: peaceful")
        ),
        0 | _ => (),
    }

    v0!("DDNS with DNS has started {}", Emoji("✨", ""));
    v1!("DNS: {}", info(&options.dns));
    v1!(
        "Will request public IP from [{}] every [{}] seconds.",
        info(&options.ip_provider),
        info(&options.interval)
    );

    let mut timer = time::interval(Duration::from_secs(options.interval));
    loop {
        timer.tick().await;

        v2!(
            "Requesting {} for public IP ...",
            info(&options.ip_provider)
        );

        let started_at = SystemTime::now();
        let ret = ip_provider::get_ip(&options.ip_provider).await;
        match ret {
            Ok(ip) => {
                let duration = SystemTime::now().duration_since(started_at)
                    .expect("Clock may have gone backwards");

        v0!(
            "{} Successfully updated dns record! (in {}ms)",
            success("✔"),
            info(duration.as_millis())
        );
    }
}

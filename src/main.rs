use clap::Clap;
use console::Emoji;
use dotenv::dotenv;
use std::time::{Duration, SystemTime};
use tokio::time;

use crate::util::is_ip;
use util::{
    check, cross, error, info, set_verbosity_level, success, v0, v1, v2, verbose_log, warn,
};

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
        long = "interval",
        default_value = "60",
        about = "The interval to request IP provider for public IP."
    )]
    interval: u64,
    #[clap(
        long = "domain",
        required = true,
        about = "The domain of your DNS record."
    )]
    domain: String,
    #[clap(
        long = "record-type",
        default_value = "A",
        about = "The type of your DNS record."
    )]
    record_type: String,
    #[clap(
        long = "record-host",
        about = "The host of your DNS record, just like what your config on DNS."
    )]
    record_host: Option<String>,
    #[clap(
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
    dotenv().ok();

    // Parse options
    let options: Options = Options::parse();
    // println!("{:?}", options);

    let mut last_updated_ip: Option<String> = None;
    let mut last_updated_at: Option<SystemTime> = None;
    let update_period = Duration::from_secs(options.record_ttl.into());

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
        if let Err(e) = ret.as_ref() {
            v0!("{} {}", cross(), error(e).to_string());
        }
        let duration = SystemTime::now()
            .duration_since(started_at)
            .expect("Clock may have gone backwards");

        let ip = ret.unwrap();

        if !is_ip(&ip) {
            v0!(
                "{} Successfully updated dns record! (in {}ms)",
                cross(),
                info(duration.as_millis())
            );
            continue;
        } else {
            v0!(
                "{} Successfully got current public IP: {} (in {}ms)",
                check(),
                success(&ip),
                info(duration.as_millis())
            );
        }

        // If the last IP update is the same as the current IP and the update cycle has not yet been reached,
        // then skip.
        if last_updated_ip.is_some() && last_updated_at.is_some() {
            let since_last_updated = SystemTime::now()
                .duration_since(last_updated_at.to_owned().unwrap())
                .expect("Clock may have gone backwards");
            if last_updated_ip.as_ref().unwrap() == &ip && since_last_updated < update_period {
                v2!("Skip updating public IP.");
                continue;
            }
        }

        v2!("Requesting {} to update DNS record ...", info(&options.dns));

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

        if ret.is_ok() {
            // Save IP and SystemTime when DNS update succeeds.
            last_updated_ip = Some(ip);
            last_updated_at = Some(SystemTime::now());

            v0!(
                "{} Successfully updated dns record! (in {}ms)",
                check(),
                info(duration.as_millis())
            );
        } else {
            v0!(
                "{} Successfully updated dns record! (in {}ms)",
                cross(),
                info(duration.as_millis())
            );
        }
    }
}

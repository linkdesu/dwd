use clap::Clap;
use console::{Style, Emoji};
use tokio::time;
use std::time::{Duration, SystemTime};
use vlog::{set_verbosity_level, v0, v1, v2, verbose_log};
use std::thread::sleep;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Link Xie. <xieaolin@gmail.com>")]
struct Options {
    #[clap(short = 'd', long = "dns", default_value = "name.com", about = "The DNS who providing resolving of your host.")]
    dns: String,
    #[clap(short = 'i', long = "ip-provider", default_value = "myip.ipip.net", about = "The provider who detecting and providing your public IP.")]
    ip_provider: String,
    #[clap(short = 'I', long = "interval", default_value = "60", about = "The interval to request IP provider for public IP.")]
    interval: u64,
    #[clap(short = 't', long = "record-type", default_value = "A", about = "The type of your DNS record.")]
    record_type: String,
    #[clap(short = 'h', long = "record-host", required = true, about = "The host of your DNS record.")]
    record_host: String,
    #[clap(short = 'T', long = "record-ttl", default_value = "300", about = "The ttl of your DNS record.")]
    record_ttl: u64,
    #[clap(short = 'v', long = "verbose", parse(from_occurrences), about = "The level of log verbosity.")]
    verbose: i32,
}

#[tokio::main]
async fn main() {
    // Parse options
    let options: Options = Options::parse();
    // println!("{:?}", options);

    // Init terminal
    let error = Style::new().red();
    let warn = Style::new().yellow();
    let info = Style::new().cyan();
    let success = Style::new().green();
    // println!("This is {}", error.apply_to("error"));
    // println!("This is {}", warn.apply_to("warn"));
    // println!("This is {}", info.apply_to("info"));
    // println!("This is {}", success.apply_to("success"));

    set_verbosity_level(options.verbose as usize);
    match options.verbose {
        2 => v2!("{}{}", Emoji("⚠️ ", ""), error.apply_to("{}Log verbosity level: crazy")),
        1 => v1!("{}{}", Emoji("⚠️ ", ""), warn.apply_to("{}Log verbosity level: peaceful")),
        0 | _ => (),
    }

    v0!("DDNS with DNS has started {}", Emoji("✨", ""));
    v1!("DNS: {}", info.apply_to(&options.dns));
    v1!("Will request public IP from [{}] every [{}] seconds.",
                 info.apply_to(&options.ip_provider),
                 info.apply_to(&options.interval));

    let mut interval_day = time::interval(Duration::from_secs(options.interval));
    loop {
        v2!("Requesting {} for public IP ...", info.apply_to(&options.ip_provider));
        let started_at = SystemTime::now();

        sleep(Duration::from_millis(512));

        let duration = SystemTime::now().duration_since(started_at)
            .expect("Clock may have gone backwards");

        v0!("{} Successfully get current public IP: {} (in {}ms)",
            success.apply_to("✔"),
            success.apply_to("1.1.1.1"),
            info.apply_to(duration.as_millis()));

        interval_day.tick().await;
    }
}

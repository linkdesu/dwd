# DDNS with DNS and DDNS


For a long long time, it is really a pain on ass to connect home network. I have tried tailscale, zerotier and none of them is stable enough for daily work. But finally I could get a public IP from my ISP, though it is changing sometimes. So for high availability and practicing purpose I developed this tool to update my public IP to DNS and DDNS.

If you have the same needs and situation like me, this will be a handy tool for publishing the public IP of your home network. Then with port forwarding and VPN like wireguard, you will have the opportunity to build a robust way to connect your home network.


## Features

- Support querying public IP from multiple providers for high availability.
- Support updating multi DNS and DDNS providers for high availability.
- Support config with JSON, YAML and TOML，you may follow your heart freely.

Currently supported public IP providers:

- https://myip.ipip.net
- https://api.myip.la
- https://ip.vnet.one/check.php

Currently supported DNS and DDNS providers:

- https://name.com
- https://dynv6.com



## Get started

### Installation

You may download and build yourself:

```shell
cargo install dwd
```

Or, you may create a config first base on `config/config_example.yaml`, and then start with the public docker image:

```
docker run --rm -it --name dwd \
    -v "${PWD}/config.yaml":/app/config.yaml \
    xieaolin/dwd:latest
```


### Configuration

- Copy `config/.env.example` to `.env`, fill in the API tokens of your DNS and DDNS providers.
- Copy one of `config/config_*.json/yaml/toml` to `config.json/yaml/toml ，and then config as you like.

The fields of config explained below:

```
// DNS and DDNS providers, you may choose multiple.
dns = ["name.com", "dynv6.com"]
// IP providers, dwd will query IP one by one until it is succeed.
ip_provider = ["myip.la", "ipip.net", "vnet.one"]
// Update interval, dwd will query IP every x seconds, and see if it is changed, if it is, update the DNS and DDNS records.
interval = 300

// The configs for DNS provider name.com
[name_com]
domain = "your.example.com"
username = ""
token = ""
record_type = "A"
record_host = "your"
record_ttl = 300

// The configs for DDNS provider dynv6.com
[dynv6_com]
zone = "your.dynv6.net"
token = ""
```

### Execution

```shell
dwd 0.2.0
Link <xieaolin@gmail.com>

USAGE:
    dwd [OPTIONS] --config <CONFIG>

OPTIONS:
    -c, --config <CONFIG>    Use a config file to configure behaviors intead of the command line
                             options.
    -h, --help               Print help information
    -v, --verbose            The level of log verbosity.
    -V, --version            Print version information
```


## TODO

- [] Prebuild binaries with Github workflow.
- [] Support cloudflare.com as DNS provider.

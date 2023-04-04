![GitHub last commit](https://img.shields.io/github/last-commit/centminmod/centminmod-csf-lfd-parser) ![GitHub contributors](https://img.shields.io/github/contributors/centminmod/centminmod-csf-lfd-parser) ![GitHub Repo stars](https://img.shields.io/github/stars/centminmod/centminmod-csf-lfd-parser) ![GitHub watchers](https://img.shields.io/github/watchers/centminmod/centminmod-csf-lfd-parser) ![GitHub Sponsors](https://img.shields.io/github/sponsors/centminmod) ![GitHub top language](https://img.shields.io/github/languages/top/centminmod/centminmod-csf-lfd-parser) ![GitHub language count](https://img.shields.io/github/languages/count/centminmod/centminmod-csf-lfd-parser)

Centmin Mod CSF LFD Log Parser

Four versions - shell script, Python, Golang and Rust versions - see [benchmarks](#benchmarks).

* [`lfd-parser.sh`](#shell-script-version) - requires both MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb` and `/usr/local/nginx-dep/bin/mmdblookup` be available.
* [`lfd-parser.py`](#python-version) - requires both MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb` and `/usr/local/nginx-dep/bin/mmdblookup` be available.
* [`lfd-parser.go`](#golang-version) - requires only that MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb` be available as it uses `geoip2-golang` instead of `mmdblookup`. Supports [filtering](#filtering) options for `--ip`, `--asn` and `--info`.
* [`lfd-parser.rs`](#rust-version) - requires `Cargo.toml` file. Supports [filtering](#filtering-rust) options for `-i`, `-a`, `-d` and `-n`. Including [Standalone binary builds](#alternative-standalone-rust-binary)

All four versions parses the CSF LFD `lfd.log` log for timestamp, IP address and type but additionally does an optional IP ASN number/organization lookup if it detects local MaxMind GeoLite2 ASN database being installed. The local MaxMind GeoLite 2 ASN database will be installed and available when:

1. Centmin Mod persistent config `/etc/centminmod/custom_config.inc` set with `MM_LICENSE_KEY='YOUR_MAXMIND_LICENSEKEY'` and `MM_CSF_SRC='y'`. Where `YOUR_MAXMIND_LICENSEKEY` is your Maxmind GeoLite2 database API Key you sign up for at https://www.maxmind.com/en/geolite2/signup
2. Centmin Mod persistent config `/etc/centminmod/custom_config.inc` set with `NGINX_GEOIPTWOLITE='y'` before Nginx install or Nginx recompiles (centmin.sh menu option 4). The local MaxMind GeoLite 2 ASN database will automatically update over time.

When `NGINX_GEOIPTWOLITE='y'`, `MM_LICENSE_KEY='YOUR_MAXMIND_LICENSEKEY'` and `MM_CSF_SRC='y'` are set in Centmin Mod persistent config `/etc/centminmod/custom_config.inc`, then `mmdblookup` command will be available at `/usr/local/nginx-dep/bin/mmdblookup` and MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb`. The `lfd-parser.sh` script can then take advantage of having a local MaxMind GeoLite2 ASN database to query and lookup an IP addresses' ASN info.

```
ls -lah /usr/share/GeoIP/ | grep mmdb
-rw-r--r--    1 root root 7.8M Mar 20 17:24 GeoLite2-ASN.mmdb
-rw-r--r--    1 root root  70M Mar 20 17:25 GeoLite2-City.mmdb
-rw-r--r--    1 root root 5.6M Mar 20 17:26 GeoLite2-Country.mmdb
```

```
/usr/local/nginx-dep/bin/mmdblookup --version

  mmdblookup version 1.7.1
```
```
/usr/local/nginx-dep/bin/mmdblookup --file /usr/share/GeoIP/GeoLite2-ASN.mmdb --ip 187.1.178.101 | sed 's/<[^>]*>//g; s/^[[:space:]]*//'

{
"autonomous_system_number": 
21574 
"autonomous_system_organization": 
"Century Telecom Ltda" 
}
```

# Shell Script Version

Example output from command:

```
./lfd-parser.sh
```

```json
[
  {
    "timestamp": "Mar 26 02:37:08",
    "ip": "92.205.40.41",
    "type": "Blocked in csf",
    "asn_number": 21499,
    "asn_org": "Host Europe GmbH",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```

Corresponding `lfd.log` entries

```
egrep 'Blocked in csf|SSH login' /var/log/lfd.log | tail -3
Mar 26 02:37:08 inc2 lfd[299494]: (sshd) Failed SSH login from 92.205.40.41 (DE/Germany/-): 5 in the last 3600 secs - *Blocked in csf* [LF_SSHD]
Mar 26 02:44:29 inc2 lfd[299629]: (sshd) Failed SSH login from 117.132.192.31 (CN/China/-): 5 in the last 3600 secs - *Blocked in csf* [LF_SSHD]
Mar 26 02:44:29 inc2 lfd[299630]: 117.132.192.31 (CN/China/-), 5 distributed sshd attacks on account [root] in the last 3600 secs - *Blocked in csf* [LF_DISTATTACK]
```

```bash
./lfd-parser.sh > parsed.log
```
```
cat parsed.log | jq -r '.[] | .ip' | sort | uniq -c | sort -rn | head -n10
      2 85.152.30.138
      2 81.22.233.170
      2 80.251.216.10
      2 79.9.37.49
      2 67.205.174.220
      2 43.128.233.179
      2 41.72.219.102
      2 207.249.96.147
```
```
cat parsed.log | jq -r '.[] | "\(.ip) \(.asn_number) \(.asn_org) \(.info)"' | sort | uniq -c | sort -rn | head -n10
      1 98.159.98.85 396073 MAJESTIC-HOSTING-01 LF_DISTATTACK
      1 97.65.33.11 3549 LVLT-3549 LF_SSHD
      1 95.85.27.201 14061 DIGITALOCEAN-ASN LF_SSHD
      1 95.85.124.113 20661 State Company of Electro Communications Turkmentelecom LF_SSHD
      1 95.232.253.35 3269 Telecom Italia LF_SSHD
      1 95.152.60.98 12389 Rostelecom LF_DISTATTACK
      1 95.106.174.126 12389 Rostelecom LF_SSHD
      1 94.153.212.78 15895 Kyivstar PJSC LF_SSHD
```

# Python Version

There's also a Python version `lfd-parser.py` for Python 3.6+ and above that will parse the logs and process the ASN info way faster than `lfd-parser.sh`.

```
time python3 lfd-parser.py > parsed-python.log

real    0m1.087s
user    0m0.445s
sys     0m0.774s

time ./lfd-parser.sh > parsed.log

real    1m27.451s
user    2m40.641s
sys     0m11.483s
```

```
cat parsed-python.log | jq -r '.[] | .ip' | sort | uniq -c | sort -rn | head -n10
      2 85.152.30.138
      2 81.22.233.170
      2 80.251.216.10
      2 79.9.37.49
      2 67.205.174.220
      2 43.128.233.179
      2 41.72.219.102
      2 207.249.96.147
```
```
cat parsed-python.log | jq -r '.[] | "\(.ip) \(.asn_number) \(.asn_org) \(.info)"' | sort | uniq -c | sort -rn | head -n10
      1 98.159.98.85 396073 MAJESTIC-HOSTING-01 LF_DISTATTACK
      1 97.65.33.11 3549 LVLT-3549 LF_SSHD
      1 95.85.27.201 14061 DIGITALOCEAN-ASN LF_SSHD
      1 95.85.124.113 20661 State Company of Electro Communications Turkmentelecom LF_SSHD
      1 95.232.253.35 3269 Telecom Italia LF_SSHD
      1 95.152.60.98 12389 Rostelecom LF_DISTATTACK
      1 95.106.174.126 12389 Rostelecom LF_SSHD
      1 94.153.212.78 15895 Kyivstar PJSC LF_SSHD
```

# Golang Version

Centmin Mod install GO via `addons/golang.sh` and then exist SSH session and relogin

```
/usr/local/src/centminmod/addons/golang.sh install
```

```
mkdir lfd-parser
cd lfd-parser
go install github.com/oschwald/geoip2-golang@latest
go mod init lfd-parser
go get github.com/oschwald/geoip2-golang
# build it
go build -ldflags="-s -w" -o lfd-parser lfd-parser.go
```
Then you can run `lfd-parser`

```
time ./lfd-parser --p /var/log/lfd.log
[
  {
    "timestamp": "Mar 26 04:35:32",
    "ip": "36.248.12.38",
    "type": "Blocked in csf",
    "asn_number": 4837,
    "asn_org": "CHINA UNICOM China169 Backbone",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 04:52:33",
    "ip": "36.112.171.51",
    "type": "Blocked in csf",
    "asn_number": 4847,
    "asn_org": "China Networks Inter-Exchange",
    "info": "LF_DISTATTACK"
  },
  {
    "timestamp": "Mar 26 05:24:13",
    "ip": "54.37.196.181",
    "type": "Blocked in csf",
    "asn_number": 16276,
    "asn_org": "OVH SAS",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 05:27:54",
    "ip": "210.114.1.46",
    "type": "Blocked in csf",
    "asn_number": 4766,
    "asn_org": "Korea Telecom",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 05:30:14",
    "ip": "155.248.233.18",
    "type": "Blocked in csf",
    "asn_number": 31898,
    "asn_org": "ORACLE-BMC-31898",
    "info": "LF_SSHD"
  }
]

real    0m0.003s
user    0m0.001s
sys     0m0.001s
```

Timed comparison for `lfd-parser.py` vs `lfd-parser.sh` vs `lfd-parser` (`lfd-parser.go`)

```
time python3 lfd-parser.py /var/log/lfd.log-20230326.gz > parsed-python.log

real    0m1.088s
user    0m0.482s
sys     0m0.735s

time ./lfd-parser.sh /var/log/lfd.log-20230326.gz > parsed.log

real    1m29.289s
user    2m40.106s
sys     0m12.970s

time ./lfd-parser --p /var/log/lfd.log-20230326.gz > parsed-golang.log

real    0m0.022s
user    0m0.021s
sys     0m0.002s
```

```
/usr/bin/time --format='real: %es user: %Us sys: %Ss cpu: %P maxmem: %M KB cswaits: %w' python3 lfd-parser.py /var/log/lfd.log-20230326.gz > parsed-python.log

real: 1.13s user: 0.48s sys: 0.78s cpu: 112% maxmem: 14520 KB cswaits: 3434

/usr/bin/time --format='real: %es user: %Us sys: %Ss cpu: %P maxmem: %M KB cswaits: %w' ./lfd-parser.sh /var/log/lfd.log-20230326.gz > parsed.log

real: 86.57s user: 159.33s sys: 11.05s cpu: 196% maxmem: 18120 KB cswaits: 66934

/usr/bin/time --format='real: %es user: %Us sys: %Ss cpu: %P maxmem: %M KB cswaits: %w' ./lfd-parser --p /var/log/lfd.log-20230326.gz > parsed-golang.log

real: 0.03s user: 0.03s sys: 0.00s cpu: 105% maxmem: 15256 KB cswaits: 114
```

```
cat parsed-golang.log | jq -r '.[] | "\(.ip) \(.asn_number) \(.asn_org) \(.info)"' | sort | uniq -c | sort -rn | head -n10

      1 98.159.98.85 396073 MAJESTIC-HOSTING-01 LF_DISTATTACK
      1 97.65.33.11 3549 LVLT-3549 LF_SSHD
      1 95.85.27.201 14061 DIGITALOCEAN-ASN LF_SSHD
      1 95.85.124.113 20661 State Company of Electro Communications Turkmentelecom LF_SSHD
      1 95.232.253.35 3269 Telecom Italia LF_SSHD
      1 95.152.60.98 12389 Rostelecom LF_DISTATTACK
      1 95.106.174.126 12389 Rostelecom LF_SSHD
      1 94.153.212.78 15895 Kyivstar PJSC LF_SSHD
```

## Filtering

The Golang version `lfd-parser` also supports filtering by:

* `--ip` - filter by IP Address. Passing multiple flag instances - equivalent of `OR` filter.
* `--asn` - filter by ASN Number. Passing multiple flag instances - equivalent of `OR` filter.
* `--info` - filter by info field i.e. `LF_SSHD`, `LF_DISTATTACK`. Passing multiple flag instances - equivalent of `OR` filter.

```
./lfd-parser --p /var/log/lfd.log-20230326.gz --ip 117.132.192.31
[
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```
Support multiple flag instances `--ip 117.132.192.31 --ip 2.59.62.229`
```
./lfd-parser --p /var/log/lfd.log-20230326.gz --ip 117.132.192.31 --ip 2.59.62.229
[
  {
    "timestamp": "Mar 26 02:35:49",
    "ip": "2.59.62.229",
    "type": "Blocked in csf",
    "asn_number": 63023,
    "asn_org": "AS-GLOBALTELEHOST",
    "info": "LF_DISTATTACK"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```
```
./lfd-parser --p /var/log/lfd.log-20230326.gz --asn 9808
[
  {
    "timestamp": "Mar 22 04:11:58",
    "ip": "120.210.206.146",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]

```
Support multiple flag instances `--asn 9808 --asn 9318`
```
./lfd-parser --p /var/log/lfd.log-20230326.gz --asn 9808 --asn 9318
[
  {
    "timestamp": "Mar 19 06:37:34",
    "ip": "110.11.234.8",
    "type": "Blocked in csf",
    "asn_number": 9318,
    "asn_org": "SK Broadband Co Ltd",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```
```
./lfd-parser --p /var/log/lfd.log-20230326.gz --info LF_DISTATTACK
[
  {
    "timestamp": "Mar 26 02:35:49",
    "ip": "2.59.62.229",
    "type": "Blocked in csf",
    "asn_number": 63023,
    "asn_org": "AS-GLOBALTELEHOST",
    "info": "LF_DISTATTACK"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```

# Rust Version

```
mkdir -p /home/rusttmp
chmod 1777 /home/rusttmp
export TMPDIR=/home/rusttmp
# install Rust via rustup can be uninstalled via 
# rustup self uninstall
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cd /home
cargo new lfd_parser
cd lfd_parser
```

In directory `lfd_parser` create or edit existing `Cargo.toml` file with:

```
[package]
name = "lfd_parser"
version = "0.1.0"
edition = "2021"

[dependencies]
maxminddb = "0.17.0"
regex = "1.5.4"
serde = "1.0.130"
serde_json = "1.0.94"
clap = "3.0.0-beta.5"
flate2 = "1.0.21"
serde_derive = "1.0.130"
```

Replace the contents of the `src/main.rs` file with the Rust code below found in `lfd-parsers.rs`:


Build and run Cargo project 

In debug / development mode

```
cargo run
```

In release - this will create an optimized binary in the `target/release` directory like `./target/release/lfd_parser`.

```
cargo build --release
```

Resulting binary `/target/release/lfd_parser`

```
ls -lah ./target/release/lfd_parser
-rwxr-xr-x 2 root root 6.6M Mar 27 00:48 ./target/release/lfd_parser
```

Dependencies for built binary are system specific.

```
ldd ./target/release/lfd_parser
        linux-vdso.so.1 (0x00007ffddf9f9000)
        libc.so.6 => /lib64/libc.so.6 (0x00007f82f6af0000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f82f72c9000)
        libpthread.so.0 => /lib64/libpthread.so.0 (0x00007f82f68d0000)
        libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x00007f82f66b8000)
        libdl.so.2 => /lib64/libdl.so.2 (0x00007f82f64b4000)
```

## Help Info

Help info

```
./target/release/lfd_parser --help
Log Analyzer 

USAGE:
    lfd_parser [OPTIONS]

OPTIONS:
    -a <asn>...         Filter by ASN number
    -d <db_path>        Path to the GeoLite2 database [default: /usr/share/GeoIP/GeoLite2-ASN.mmdb]
    -h, --help          Print help information
    -i <ip>...          Filter by IP address
    -n <info>...        Filter by Info
    -p <path>           Path to the log file [default: /var/log/lfd.log]
```

Then to run the built binary

```
./target/release/lfd_parser -p /var/log/lfd.log
```

## Alternative Standalone Rust Binary

```
cd /home/lfd_parser
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/lfd_parser /usr/local/bin/
strip /usr/local/bin/lfd_parser
```
Now can access binary from `/usr/local/bin/lfd_parser`

```
/usr/local/bin/lfd_parser --help
Log Analyzer 

USAGE:
    lfd_parser [OPTIONS]

OPTIONS:
    -a <asn>...         Filter by ASN number
    -d <db_path>        Path to the GeoLite2 database [default: /usr/share/GeoIP/GeoLite2-ASN.mmdb]
    -h, --help          Print help information
    -i <ip>...          Filter by IP address
    -n <info>...        Filter by Info
    -p <path>           Path to the log file [default: /var/log/lfd.log]
```
No dependencies

```
ldd /usr/local/bin/lfd_parser
        statically linked
```

```
time /usr/local/bin/lfd_parser -p /var/log/lfd.log-20230326.gz
```

## Filtering Rust

Example commands
```
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -i 117.132.192.31
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -i 117.132.192.31 -i 2.59.62.229
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -a 9808
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -a 9808 -a 9318
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -n LF_DISTATTACK
```
Example outputs
```
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -i 117.132.192.31
[
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```
```
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -i 117.132.192.31 -i 2.59.62.229
[
  {
    "timestamp": "Mar 26 02:35:49",
    "ip": "2.59.62.229",
    "type": "Blocked in csf",
    "asn_number": 63023,
    "asn_org": "AS-GLOBALTELEHOST",
    "info": "LF_DISTATTACK"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```
```
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -a 9808
[
  {
    "timestamp": "Mar 22 04:11:58",
    "ip": "120.210.206.146",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```
```
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -a 9808 -a 9318
[
  {
    "timestamp": "Mar 19 06:37:34",
    "ip": "110.11.234.8",
    "type": "Blocked in csf",
    "asn_number": 9318,
    "asn_org": "SK Broadband Co Ltd",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_SSHD"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```
```
./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz -n LF_DISTATTACK

[
  {
    "timestamp": "Mar 26 02:35:49",
    "ip": "2.59.62.229",
    "type": "Blocked in csf",
    "asn_number": 63023,
    "asn_org": "AS-GLOBALTELEHOST",
    "info": "LF_DISTATTACK"
  },
  {
    "timestamp": "Mar 26 02:44:29",
    "ip": "117.132.192.31",
    "type": "Blocked in csf",
    "asn_number": 9808,
    "asn_org": "China Mobile Communications Group Co., Ltd.",
    "info": "LF_DISTATTACK"
  }
]
```

## Benchmarks

Parsing CSF Firewall's LFD log file at `/var/log/lfd.log-20230326.gz`

```
ls -lah /var/log/lfd.log*
-rw------- 1 root root 28K Mar 27 00:53 /var/log/lfd.log
-rw------- 1 root root 24K Mar 26 02:44 /var/log/lfd.log-20230326.gz
```

Timed comparison for `lfd-parser.py` vs `lfd-parser.sh` vs `lfd-parser` (`lfd-parser.go`) vs `/target/release/lfd_parser` (rust)

```
time python3 lfd-parser.py /var/log/lfd.log-20230326.gz > parsed-python.log

real    0m1.003s
user    0m0.495s
sys     0m0.647s

time ./lfd-parser.sh /var/log/lfd.log-20230326.gz > parsed.log

real    1m26.528s
user    2m39.696s
sys     0m10.917s

time ./lfd-parser --p /var/log/lfd.log-20230326.gz > parsed-golang.log

real    0m0.023s
user    0m0.021s
sys     0m0.004s

time ./target/release/lfd_parser -p /var/log/lfd.log-20230326.gz > parsed-rust.log

real    0m0.009s
user    0m0.004s
sys     0m0.005s
```

| Language | Script/Executable                | Speed-up Factor | Real Time  | User Time  | System Time |
|----------|----------------------------------|-----------------|------------|------------|-------------|
| Python   | `python3 lfd-parser.py`          | 86.29x          | 0m1.003s   | 0m0.495s   | 0m0.647s    |
| Shell    | `./lfd-parser.sh`                | 1.00x           | 1m26.528s  | 2m39.696s  | 0m10.917s   |
| Golang   | `./lfd-parser`                   | 3762.09x        | 0m0.023s   | 0m0.021s   | 0m0.004s    |
| Rust     | `./target/release/lfd_parser`    | 9614.22x        | 0m0.009s   | 0m0.004s   | 0m0.005s    |

Querying the `parsed-rust.log`

```
cat parsed-rust.log | jq -r '.[] | "\(.ip) \(.asn_number) \(.asn_org) \(.info)"' | sort | uniq -c | sort -rn | head -n10
      1 98.159.98.85 396073 MAJESTIC-HOSTING-01 LF_DISTATTACK
      1 97.65.33.11 3549 LVLT-3549 LF_SSHD
      1 95.85.27.201 14061 DIGITALOCEAN-ASN LF_SSHD
      1 95.85.124.113 20661 State Company of Electro Communications Turkmentelecom LF_SSHD
      1 95.232.253.35 3269 Telecom Italia LF_SSHD
      1 95.152.60.98 12389 Rostelecom LF_DISTATTACK
      1 95.106.174.126 12389 Rostelecom LF_SSHD
      1 94.153.212.78 15895 Kyivstar PJSC LF_SSHD
```
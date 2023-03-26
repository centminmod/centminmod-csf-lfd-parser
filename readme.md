Centmin Mod CSF LFD Log Parser

Three versions - shell script, Python and Golang versions:

* [`lfd-parser.sh`](#shell-script-version) - requires both MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb` and `/usr/local/nginx-dep/bin/mmdblookup` be available.
* [`lfd-parser.py`](#python-version) - requires both MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb` and `/usr/local/nginx-dep/bin/mmdblookup` be available.
* [`lfd-parser.go`](#golang-version) - requires only that MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb` be available as it uses `geoip2-golang` instead of `mmdblookup`. Supports [filtering](#filtering) options for `--ip`, `--asn` and `--info`.

All tree versions parses the CSF LFD `lfd.log` log for timestamp, IP address and type but additionally does an optional IP ASN number/organization lookup if it detects local MaxMind GeoLite2 ASN database being installed. The local MaxMind GeoLite 2 ASN database will be installed and available when Centmin Mod persistent config `/etc/centminmod/custom_config.inc` set with `NGINX_GEOIPTWOLITE='y'` before Nginx install or Nginx recompiles (centmin.sh menu option 4). The local MaxMind GeoLite 2 ASN database will automatically update over time.

When `NGINX_GEOIPTWOLITE='y'` is set in Centmin Mod persistent config `/etc/centminmod/custom_config.inc`, then `mmdblookup` command will be available at `/usr/local/nginx-dep/bin/mmdblookup` and MaxMind GeoLite2 ASN database at `/usr/share/GeoIP/GeoLite2-ASN.mmdb`. The `lfd-parser.sh` script can then take advantage of having a local MaxMind GeoLite2 ASN database to query and lookup an IP addresses' ASN info.


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

* `--ip` - filter by IP Address
* `--asn` - filter by ASN Number
* `--info` - filter by info field i.e. `LF_SSHD`, `LF_DISTATTACK`

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
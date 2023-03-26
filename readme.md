Centmin Mod CSF LFD Log Parser

Parses the CSF LFD `lfd.log` log for timestamp, IP address and type but additional does an optional IP ASN number/organization lookup if it detects local MaxMind GeoLite2 ASN database being installed. The local MaxMind GeoLite 2 ASN database will be installed and available when Centmin Mod persistent config `/etc/centminmod/custom_config.inc` set with `NGINX_GEOIPTWOLITE='y'` before Nginx install or Nginx recompiles (centmin.sh menu option 4). The local MaxMind GeoLite 2 ASN database will automatically update over time.

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

Example output from:

```
./lfd-parser.sh
```

```json
[
  {
    "timestamp": "Mar 19 04:20:10",
    "ip": "187.1.178.101",
    "type": "Blocked in csf",
    "asn_number": "21574",
    "asn_org": "Century Telecom Ltda"
  },
  {
    "timestamp": "Mar 19 04:20:10",
    "ip": "103.232.121.81",
    "type": "Blocked in csf",
    "asn_number": "56150",
    "asn_org": "Viet Solutions Services Trading Company Limited"
  },
  {
    "timestamp": "Mar 25 21:18:53",
    "ip": "61.177.173.41",
    "type": "Blocked in csf",
    "asn_number": "4134",
    "asn_org": "Chinanet"
  }
]
```

```bash
./lfd-parser.sh > parsed.log
```
```
cat parsed.log | jq -r '.[] | .ip' | sort | uniq -c | sort -rn | head -n10
      2 81.22.233.170
      2 80.251.216.10
      2 79.9.37.49
      2 67.205.174.220
      2 43.128.233.179
      2 41.72.219.102
      2 207.249.96.147
      2 190.78.192.247
```
```
cat parsed.log | jq -r '.[] | "\(.ip) \(.asn_number) \(.asn_org)"' | sort | uniq -c | sort -rn | head -n10
      2 81.22.233.170 48146 Triple A Fibra S.L.
      2 80.251.216.10 21887 FIBER-LOGIC
      2 79.9.37.49 3269 Telecom Italia
      2 67.205.174.220 14061 DIGITALOCEAN-ASN
      2 43.128.233.179 132203 Tencent Building, Kejizhongyi Avenue
      2 41.72.219.102 30844 Liquid Telecommunications Ltd
      2 207.249.96.147 13579 INFOTEC CENTRO DE INVESTIGACION E INNOVACION EN TECNOLOGIAS DE LA INFORMACION Y COMUNICACION
      2 190.78.192.247 8048 CANTV Servicios, Venezuela
```
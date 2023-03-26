#!/usr/bin/env python3
import os
import re
import sys
import gzip
import json
import subprocess

mmdblookup_bin = "/usr/local/nginx-dep/bin/mmdblookup"
asn_database = "/usr/share/GeoIP/GeoLite2-ASN.mmdb"
debug = False

def get_asn_info(ip: str) -> dict:
    try:
        output = subprocess.check_output([mmdblookup_bin, "--file", asn_database, "--ip", ip])
        output = output.decode('utf-8').strip()
        asn_number = re.search(r'"autonomous_system_number":\s+(\d+)', output)
        asn_org = re.search(r'"autonomous_system_organization":\s+"([^"]+)', output)

        asn_number = int(asn_number.group(1)) if asn_number else None
        asn_org = asn_org.group(1) if asn_org else None
    except (subprocess.CalledProcessError, KeyError, ValueError):
        asn_number = None
        asn_org = None

    return {"asn_number": asn_number, "asn_org": asn_org}

def process_line(line: str) -> dict:
    timestamp = f"{line[0]} {line[1]} {line[2]}"
    ip = next((x for x in line if re.match(r'[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+', x)), '')
    type_pattern = r'\*[^*]+\*'
    type_match = re.search(type_pattern, ' '.join(line))
    type = type_match.group(0)[1:-1] if type_match else ''
    info_pattern = r'\[[^\]]+\]$'
    info_match = re.search(info_pattern, ' '.join(line))
    info = info_match.group(0)[1:-1] if info_match else ''
    
    if debug:
        print(f"Processing line: {line}")
        print(f"IP address: {ip}")
    
    asn_info = get_asn_info(ip)
    
    if debug:
        print(f"ASN info: {asn_info}")
        print(f"Info: {info}")
    
    json_obj = {
        "timestamp": timestamp,
        "ip": ip,
        "type": type,
        "asn_number": asn_info["asn_number"],
        "asn_org": asn_info["asn_org"],
        "info": info
    }

    return json_obj

if __name__ == "__main__":
    if len(sys.argv) > 1:
        logfile = sys.argv[1]
    else:
        logfile = "/var/log/lfd.log"

    if not os.path.isfile(mmdblookup_bin) or not os.path.isfile(asn_database):
        print("mmdblookup binary or ASN database not found. Exiting.")
        sys.exit(1)

    if os.path.isfile(logfile):
        file_opener = gzip.open if logfile.endswith('.gz') else open
        with file_opener(logfile, 'rt') as f:
            lines = [l.strip().split() for l in f.readlines() if 'Blocked in csf' in l or 'SSH login' in l]
        output = [process_line(line) for line in lines]
        print(json.dumps(output, indent=2))
    else:
        print(f"Log file {logfile} not found.")


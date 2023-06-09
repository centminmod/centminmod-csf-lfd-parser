#!/bin/bash
#####################################################################
# CSF Firewall LFD log parser for Centmin Mod LEMP stack
#####################################################################
# path to CSF Firewall LFD log
logfile=/var/log/lfd.log
# available when Centmin Mod persistent config 
# /etc/centminmod/custom_config.inc set with
# NGINX_GEOIPTWOLITE='y' before Nginx install
mmdblookup_bin=/usr/local/nginx-dep/bin/mmdblookup
asn_database=/usr/share/GeoIP/GeoLite2-ASN.mmdb

# Check for required commands and install them if not found
if ! command -v jq >/dev/null 2>&1; then
  echo "jq not found. Installing..."
  yum install -y jq
fi

if ! command -v parallel >/dev/null 2>&1; then
  echo "parallel not found. Installing..."
  yum install -y parallel
fi

if [ "$#" -gt 0 ]; then
  logfile="$1"
else
  logfile="/var/log/lfd.log"
fi

function process_line() {
  line="$1"
  timestamp=$(echo "$line" | jq -r '.timestamp')
  ip=$(echo "$line" | jq -r '.ip')
  type=$(echo "$line" | jq -r '.type')
  info=$(echo "$line" | jq -r '.info')

  # Look up the ASN information for the IP using mmdblookup
  asn_info=$(/usr/local/nginx-dep/bin/mmdblookup --file /usr/share/GeoIP/GeoLite2-ASN.mmdb --ip "$ip" | tr -d '\n' | sed 's/<[^>]*>//g; s/^[[:space:]]*//; s/\([0-9]\)\s\+\("[a-z_]*"\)/\1, \2/')
  asn_number=$(echo "$asn_info" | jq -r '.autonomous_system_number')
  asn_org=$(echo "$asn_info" | jq -r '.autonomous_system_organization')

  # Build a JSON object with the parsed data and ASN information
  json_obj=$(echo '{}' | jq --arg timestamp "$timestamp" --arg ip "$ip" --arg type "$type" --arg asn_number "$asn_number" --arg asn_org "$asn_org" --arg info "$info" '.timestamp = $timestamp | .ip = $ip | .type = $type | .asn_number = $asn_number | .asn_org = $asn_org | .info = $info')

  echo "$json_obj"
}

export -f process_line

if [ -x "${mmdblookup_bin}" ] && [ -e "${asn_database}" ]; then
  json1=$(zcat -f "$logfile" | grep -E 'Blocked in csf|SSH login' | awk 'BEGIN { print "[" } {
    month = $1;
    date = $2;
    time = $3;
    timestamp = month " " date " " time;
    ip = "";
    type = "";
    for (i = 1; i <= NF; i++) {
      if ($i ~ /[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/ && ip == "") {
        ip = $i;
      }
      if (match($0, /\*[^*]+\*/)) {
        type = substr($0, RSTART+1, RLENGTH-2);
      }
      if (match($0, /\[[^]]+\]$/)) {
        info = substr($0, RSTART+1, RLENGTH-2);
      }
    }
    if (NR > 1) printf(",\n");
    printf("{\"timestamp\": \"%s\", \"ip\": \"%s\", \"type\": \"%s\", \"info\": \"%s\"}", timestamp, ip, type, info);
  } END { print "\n]" }')

  output=$(echo "$json1" | jq -c '.[]' | parallel --will-cite -j "$(nproc)" --line-buffer process_line | jq -s '.')
  # Output the final JSON array
  echo "$output"
else
  zcat -f "$logfile" | grep -E 'Blocked in csf|SSH login' | awk 'BEGIN { print "[" } {
    month = $1;
    date = $2;
    time = $3;
    timestamp = month " " date " " time;
    ip = "";
    type = "";
    for (i = 1; i <= NF; i++) {
      if ($i ~ /[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/ && ip == "") {
        ip = $i;
      }
      if (match($0, /\*[^*]+\*/)) {
        type = substr($0, RSTART+1, RLENGTH-2);
      }
      if (match($0, /\[[^]]+\]$/)) {
        info = substr($0, RSTART+1, RLENGTH-2);
      }
    }
    if (NR > 1) printf(",\n");
    printf("{\"timestamp\": \"%s\", \"ip\": \"%s\", \"type\": \"%s\", \"info\": \"%s\"}", timestamp, ip, type, info);
  } END { print "\n]" }'
fi
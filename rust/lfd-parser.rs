use std::io::{BufRead, BufReader};
use std::fs::File;
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;

use clap::{Arg, App, Values};
use flate2::bufread::GzDecoder;
use maxminddb::geoip2;
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct LogEntry {
    timestamp: String,
    ip: String,
    r#type: String,
    asn_number: Option<u32>,
    asn_org: Option<String>,
    info: String,
}

fn main() {
    let matches = App::new("Log Analyzer")
        .arg(Arg::with_name("path")
            .short('p') // Use single quotes for characters
            .default_value("/var/log/lfd.log")
            .help("Path to the log file")
            .takes_value(true))
        .arg(Arg::with_name("ip")
            .short('i') // Use single quotes for characters
            .help("Filter by IP address")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("asn")
            .short('a') // Use single quotes for characters
            .help("Filter by ASN number")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("info")
            .short('n') // Use single quotes for characters
            .help("Filter by Info")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("db_path")
            .short('d') // Use single quotes for characters
            .default_value("/usr/share/GeoIP/GeoLite2-ASN.mmdb")
            .help("Path to the GeoLite2 database")
            .takes_value(true))
        .get_matches();

    let log_file_path = matches.value_of("path").unwrap();

    let ip_filter: Vec<_> = matches.values_of("ip").unwrap_or(Values::default()).collect();
    let asn_filter: Vec<u32> = matches.values_of("asn")
        .unwrap_or(Values::default())
        .filter_map(|x| u32::from_str(x).ok())
        .collect();
    let info_filter: Vec<_> = matches.values_of("info").unwrap_or(Values::default()).collect();

    let file = File::open(log_file_path).expect("Error opening file");

    let reader: Box<dyn BufRead> = if log_file_path.ends_with(".gz") {
        Box::new(BufReader::new(GzDecoder::new(BufReader::new(file))))
    } else {
        Box::new(BufReader::new(file))
    };

    let geoip_database_path = matches.value_of("db_path").unwrap();
    let asn_db = match maxminddb::Reader::open_readfile(geoip_database_path) {
        Ok(db) => db,
        Err(_) => {
            eprintln!(
                "Error: Could not open the ASN database file at '{}'. Please make sure the file exists and is accessible.",
                geoip_database_path
            );
            std::process::exit(1);
        }
    };

    let timestamp_re = Regex::new(r"^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}").unwrap();
    let ip_re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    let type_re = Regex::new(r"\*[^*]+\*").unwrap();
    let info_re = Regex::new(r"\[[^\]]+\]$").unwrap();
    // Add the new regex for matching the attacker IP address in cluster mode.
    let attacker_ip_re = Regex::new(r"DENY\s+(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    let mut entries: Vec<LogEntry> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        if line.contains("Blocked in csf") || line.contains("SSH login") {
            let timestamp = timestamp_re.find(&line).map(|m| m.as_str()).unwrap_or("").to_string();
            // Modify the `ip` variable assignment.
            let ip = if line.contains("Cluster member") {
                attacker_ip_re
                    .find(&line)
                    .map(|m| m.as_str().trim_start_matches("DENY").trim())
                    .unwrap_or("")
                    .to_string()
            } else {
                ip_re
                    .find(&line)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .to_string()
            };
            let entry_type = type_re.find(&line).map(|m| m.as_str()).unwrap_or("").trim_matches('*').to_string();
            let info = info_re.find(&line).map(|m| m.as_str()).unwrap_or("").trim_matches('[').trim_matches(']').to_string();

            let (asn_number, asn_org) = if let Ok(ip_addr) = IpAddr::from_str(&ip) {
                if let Ok(asn) = asn_db.lookup::<geoip2::Asn>(ip_addr) {
                    (asn.autonomous_system_number, asn.autonomous_system_organization.map(|org| org.to_string()))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            let entry = LogEntry {
                timestamp,
                ip,
                r#type: entry_type,
                asn_number,
                asn_org,
                info,
            };

            if (ip_filter.is_empty() || ip_filter.iter().any(|&x| x == entry.ip)) &&
                (asn_filter.is_empty() || entry.asn_number.map_or(false, |asn| asn_filter.contains(&asn))) &&
                (info_filter.is_empty() || info_filter.iter().any(|&x| x == entry.info)) {
                entries.push(entry);
            }
        }
    }

    if entries.is_empty() {
        println!("No entries found.");
    } else {
        let json_data = serde_json::to_string_pretty(&entries).expect("Error encoding JSON");
        println!("{}", json_data);
    }
}
package main

import (
  "bufio"
  "compress/gzip"
  "encoding/json"
  "flag"
  "fmt"
  "github.com/oschwald/geoip2-golang"
  "io"
  "net"
  "os"
  "regexp"
  "strings"
)

type LogEntry struct {
  Timestamp string `json:"timestamp"`
  IP        string `json:"ip"`
  Type      string `json:"type"`
  ASNNumber uint   `json:"asn_number"`
  ASNOrg    string `json:"asn_org"`
  Info      string `json:"info"`
}

func main() {
  // Set up command line flag
  logFilePath := flag.String("p", "/var/log/lfd.log", "Path to the log file")
  ipFilter := flag.String("ip", "", "Filter by IP address")
  asnFilter := flag.Uint("asn", 0, "Filter by ASN number")
  infoFilter := flag.String("info", "", "Filter by Info")
  flag.Parse()

  // Open the log file
  var reader io.Reader
  file, err := os.Open(*logFilePath)
  if err != nil {
    fmt.Printf("Error opening file: %s\n", err)
    os.Exit(1)
  }
  defer file.Close()

  // Check if the file is gzip compressed
  if strings.HasSuffix(*logFilePath, ".gz") {
    gzreader, err := gzip.NewReader(file)
    if err != nil {
      fmt.Printf("Error opening gzip file: %s\n", err)
      os.Exit(1)
    }
    defer gzreader.Close()
    reader = gzreader
  } else {
    reader = file
  }

  // Read ASN database
  asnDB, err := geoip2.Open("/usr/share/GeoIP/GeoLite2-ASN.mmdb")
  if err != nil {
    fmt.Printf("Error opening ASN database: %s\n", err)
    os.Exit(1)
  }
  defer asnDB.Close()

  // Process log entries
  entries := []LogEntry{}
  scanner := bufio.NewScanner(reader)
  for scanner.Scan() {
    line := scanner.Text()

    if strings.Contains(line, "Blocked in csf") || strings.Contains(line, "SSH login") {
      entry := processLine(line, asnDB)

      if (*ipFilter == "" || entry.IP == *ipFilter) &&
        (*asnFilter == 0 || entry.ASNNumber == *asnFilter) &&
        (*infoFilter == "" || entry.Info == *infoFilter) {
        entries = append(entries, entry)
      }
    }
  }

  if err := scanner.Err(); err != nil {
    fmt.Printf("Error reading file: %s\n", err)
    os.Exit(1)
  }

  jsonData, err := json.MarshalIndent(entries, "", "  ")
  if err != nil {
    fmt.Printf("Error encoding JSON: %s\n", err)
    os.Exit(1)
  }

  fmt.Println(string(jsonData))
}

func processLine(line string, asnDB *geoip2.Reader) LogEntry {
  timestampRe := regexp.MustCompile(`^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}`)
  ipRe := regexp.MustCompile(`\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}`)
  typeRe := regexp.MustCompile(`\*[^*]+\*`)
  infoRe := regexp.MustCompile(`\[[^\]]+\]$`)

  timestamp := timestampRe.FindString(line)
  ip := ipRe.FindString(line)
  entryType := strings.Trim(typeRe.FindString(line), "*")
  info := strings.Trim(infoRe.FindString(line), "[]")

  asnNumber := uint(0)
  asnOrg := ""

  if ip != "" {
    ipAddr := net.ParseIP(ip)
    asn, err := asnDB.ASN(ipAddr)
    if err == nil {
      asnNumber = asn.AutonomousSystemNumber
      asnOrg = asn.AutonomousSystemOrganization
    }
  }

  entry := LogEntry{
    Timestamp: timestamp,
    IP:        ip,
    Type:      entryType,
    ASNNumber: asnNumber,
    ASNOrg:    asnOrg,
    Info:      info,
  }
  return entry
}
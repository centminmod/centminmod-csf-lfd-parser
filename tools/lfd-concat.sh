#!/bin/bash
####################################################################
# concat lfd.log logrotated + non logrotated logs into single 
# lfd-concat.log for parsing benchmarks
####################################################################
# Create or clear the final concatenated log file
> /var/log/lfd-concat.log

# Loop through each log file sorted by modification time
for log_file in $(ls -lt --time-style=+"%s" /var/log/lfd.log* | sort -k6,6n | awk '{print $NF}'); do
    if [[ $log_file == *.gz ]]; then
        # Unzip and append to the final log file
        zcat $log_file >> /var/log/lfd-concat.log
    else
        # Directly append to the final log file
        cat $log_file >> /var/log/lfd-concat.log
    fi
done
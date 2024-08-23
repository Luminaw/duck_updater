#!/bin/bash

# Get the directory path of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

PROGRAM_PATH="$SCRIPT_DIR/../target/release/duck_updater"

# Set the desired schedule (every 15 minutes in this example)
CRON_SCHEDULE="*/15 * * * *"

# Create a temporary file for the cron job
CRON_TEMP_FILE=$(mktemp)

# Write the cron job command to the temporary file
echo "$CRON_SCHEDULE cd $SCRIPT_DIR/../ && $PROGRAM_PATH >> /var/log/duck_updater.log 2>&1" > "$CRON_TEMP_FILE"

# Install the new cron job
crontab "$CRON_TEMP_FILE"
rm "$CRON_TEMP_FILE"

echo "Cron job created successfully."
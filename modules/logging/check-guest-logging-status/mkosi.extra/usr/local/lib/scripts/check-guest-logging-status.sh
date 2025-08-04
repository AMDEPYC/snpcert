#!/bin/bash

REMOTE_JOURNAL_DIR="/var/log/journal/guest-logs/"
SERVICE_NAME="systemd-journal-upload.service"

# Check if systemd-journal-remote is running
if ! systemctl is-active --quiet systemd-journal-remote; then
    echo -e "systemd-journal-remote service is NOT active on the host."
    exit 1
fi

echo "systemd-journal-remote service is active on the host."

if [ ! -d "$REMOTE_JOURNAL_DIR" ]; then
    echo -e "No remote journal directory found at $REMOTE_JOURNAL_DIR"
    exit 2
fi

found=0
for guest_dir in "$REMOTE_JOURNAL_DIR"; do
    [ -d "$guest_dir" ] || continue
    # Grab the last systemd-journal-upload service status entry
    status_entry=$(journalctl -D "$guest_dir" -u "$SERVICE_NAME" -o json -n 50 2>/dev/null | grep -E '"MESSAGE|^{' | tail -n 100 )

    # Analyze for success or failure states
    success_msg=$(echo "$status_entry" | grep -i "Started" | head -1 | jq '.MESSAGE')
    failure_msg=$(echo "$status_entry" | grep -i "Failed\|failure" | head -1 | jq '.MESSAGE')

    if [ -n "$failure_msg" ]; then
        echo -e "$SERVICE_NAME service fails on the SNP guest!!!"
        echo -e "$failure_msg"
	    exit 3
    elif [ -n "$success_msg" ]; then
        echo -e "Started $SERVICE_NAME service on the SNP guest"
    else
        echo -e "No guest log found for $SERVICE_NAME"
	    exit 4
    fi

    found=1
done

if [ $found -eq 0 ]; then
    echo "No guest journals found in $REMOTE_JOURNAL_DIR on the host."
fi

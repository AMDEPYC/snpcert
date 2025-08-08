#!/bin/bash

REMOTE_JOURNAL_DIR="/var/log/journal/guest-logs/"
SERVICE_NAME="systemd-journal-upload.service"

# Check if systemd-journal-remote is running
if ! systemctl is-active --quiet systemd-journal-remote; then
    echo -e "systemd-journal-remote service is NOT active on the host."
    exit 1
fi

echo "systemd-journal-remote service is active on the host."

# Check if the guest directory exists on the host
if [ ! -d "$REMOTE_JOURNAL_DIR" ]; then
    echo -e "No remote journal directory found at $REMOTE_JOURNAL_DIR"
    exit 2
fi

# Check if host can read guest service from the  guest journal directory
guest_dir=$REMOTE_JOURNAL_DIR
status_entry=$(journalctl -D "$guest_dir" -u "$SERVICE_NAME" -o json )

if [ -z "${status_entry}" ]; then
    echo -e "Failed to read guest journal on the host from \"${REMOTE_JOURNAL_DIR}\" !!"
    echo -e "\"${REMOTE_JOURNAL_DIR}\" guest journal directory is empty!!"
    echo -e "systemd-journal-upload.service may not be active and running on the guest"
    echo -e "Please check for the systemd-journal-upload.service status on the guest"
    exit 3
fi

echo -e "Host can successfully read the  guest journal logs !!!"

#!/bin/bash

GUEST_JOURNAL_LOCATION="/var/log/journal/guest-logs"

echo -e "\nChecking SNP Guest Attestation status..."

# Get the snpguest-attestation service log from the guest journal
guest_service_log=$(journalctl -D "${GUEST_JOURNAL_LOCATION}" -u snpguest-attestation.service -o cat)

# Extract lines containing 'error' (case-insensitive)
error_lines=$(echo "$guest_service_log" | grep -i "error")

# Replace everything up to and including 'error' with just 'error'
attestation_error_log=$(echo "$error_lines" | sed -E 's/.*([Ee][Rr][Rr][Oo][Rr])/Error/i')

# Check for any Attestation errors
if [  -n "${attestation_error_log}" ]; then
    echo -e "\nERROR: SNP Guest Attestation fails!! \n" >&2
    echo -e "${attestation_error_log}" >&2
    exit 1
fi

# Access snpguest-attestation service real-time log on the host
echo -e "\nDisplay the snpguest attestation service of the active SNP Guest"

# Add SNP guest's snpguest-attestation service log into the host's launch-guest service
echo -e "\n${guest_service_log}"
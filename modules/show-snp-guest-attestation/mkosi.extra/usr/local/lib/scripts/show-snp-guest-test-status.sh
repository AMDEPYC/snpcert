#!/bin/bash

GUEST_JOURNAL_LOCATION="/var/log/journal/guest-logs"

echo -e "\nView SNP guest tests status..."

# Get the snpguest-ok service log from the guest journal
guest_service_log=$(journalctl -D "${GUEST_JOURNAL_LOCATION}" -u snpguest-ok.service -o cat)

# Extract lines containing 'error' (case-insensitive)
error_lines=$(echo "${guest_service_log}" | grep -i "error")

# Replace everything up to and including 'error' with just 'error'
snpguest_test_error_log=$(echo "${error_line}" | sed -E 's/.*([Ee][Rr][Rr][Oo][Rr])/Error/i')

# Check for any SNP guest test errors
if [  -n "${snpguest_test_error_log}" ]; then
    echo -e "\nERROR: SNP Guest test(s) fails!! \n" >&2
    echo -e "${snpguest_test_error_log}" >&2
    exit 1
fi

# Access snpguest-ok service real-time log on the host
echo -e "\nDisplay the SNP guest test(s) service log of the active SNP Guest"

# Add SNP guest's snpguest-ok service log into the host's launch-guest service
echo -e "\n${guest_service_log}"
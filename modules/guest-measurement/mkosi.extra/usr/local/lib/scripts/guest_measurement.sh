#!/bin/bash

set -euo pipefail

EFI_PATH="/usr/local/lib/guest-image/guest.efi"
MEASUREMENT_FILE="/usr/local/lib/guest-image/guest_measurement.txt"

# Check which OVMF binary to use
for OVMF_PATH in /usr/share/ovmf/OVMF.amdsev.fd /usr/share/edk2/ovmf/OVMF.amdsev.fd; do
  [ -f "$OVMF_PATH" ] && break
done

if [ ! -f "$OVMF_PATH" ]; then
    echo "AMDSEV compatible OVMF is not present, can't calculate measurement"
    exit 1
fi

snpguest generate measurement \
    --vcpu-type EPYC-v4 \
    --ovmf "$OVMF_PATH" \
    --kernel "$EFI_PATH" \
    --output-format hex \
    --measurement-file "$MEASUREMENT_FILE"

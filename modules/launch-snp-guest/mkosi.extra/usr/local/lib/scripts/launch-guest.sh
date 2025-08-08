#!/bin/bash

PKG_OVMF_PATH="/usr/share/edk2/ovmf/OVMF.amdsev.fd"
BUILD_OVMF_PATH="/usr/share/build/ovmf/OVMF.fd"
EFI_PATH="/usr/local/lib/guest-image/guest.efi"
MEASUREMENT_FILE="/usr/local/lib/guest-image/guest_measurement.txt"
GUEST_ERROR_LOG="/tmp/guest-error.log"

# Check which OVMF binary to use
if [ -f "$PKG_OVMF_PATH" ]; then
    OVMF_PATH="$PKG_OVMF_PATH"
    echo "Using packaged OVMF binary: $OVMF_PATH"
elif [ -f "$BUILD_OVMF_PATH" ]; then
    OVMF_PATH="$BUILD_OVMF_PATH"
    echo "Using built OVMF binary: $OVMF_PATH"
else
    echo "Error: Could not find OVMF binary at $PKG_OVMF_PATH or $BUILD_OVMF_PATH" >&2
    exit 1
fi


# Convert Measurement to the appropriate sha format to pass in as host data
calculated_measurement_hex=$(awk -F "0x" '{print $2}' "${MEASUREMENT_FILE}" )
guest_measurement_sha256sum=$(echo "${calculated_measurement_hex}" | sha256sum | cut -d ' ' -f 1 | xxd -r -p | base64 )

# Clean up the error trace before QEMU guest launch
truncate -s 0 ${GUEST_ERROR_LOG}

# Launch the SNP guest in background
exec qemu-system-x86_64 \
  -enable-kvm \
  -cpu EPYC-v4 \
  -machine memory-encryption=sev0 \
  -monitor none \
  -display none \
  -object memory-backend-memfd,id=ram1,size=2048M \
  -machine memory-backend=ram1 \
  -object sev-snp-guest,id=sev0,cbitpos=51,reduced-phys-bits=1,kernel-hashes=on,host-data="${guest_measurement_sha256sum}" \
  -bios ${OVMF_PATH} \
  -kernel ${EFI_PATH} 2> ${GUEST_ERROR_LOG}

echo -e "\nSNP Guest boot is in progress ..."
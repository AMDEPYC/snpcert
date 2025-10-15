The typical way to run tests is fully automated, including running guests automatically. However if you're developing tests or isolating an issue, you may find it useful to run guests manually. The following instructions assume you are running on AMD EPYC hardware that has been fully enabled for SEV.

# Prerequisites

Ensure that you are running on an AMD EPYC hardware & kernel combination that is fully enabled for SEV 3.0 (SNP). See the [AMD SEV Tuning Guide](https://www.amd.com/content/dam/amd/en/documents/epyc-technical-docs/tuning-guides/58207-using-sev-with-amd-epyc-processors.pdf) for supported kernel levels and firmware enablement instructions.

You can verify this by running the following command and resolving any failures:

```
snphost ok
```

If you are using a host image built by this repository, snphost is already installed in /usr/local/bin/snphost. If you are not, you will need to download it here: https://github.com/virtee/snphost

# How to Run

1. Download or build artifacts for your relevant guest distro.

Guest images are available for download in sev-certify release assets: https://github.com/AMDEPYC/sev-certify/releases. Unzip them if necessary.

Alternatively, images can be built by cloning this repository and running `mkosi` on the appropriate directory. Target any of the [`image/` directories](https://github.com/AMDEPYC/sev-certify/tree/main/images) to build those distro-specific artifacts:

```
sudo mkosi --image-id=guest-fedora-41 -C images/guest-fedora-41 build
```

Resulting image, kernel, boot ramfs will be deposited in the targeted directory.

2. <ins>**Launch SNP Guest:** </ins>   Run an SNP guest with the direct boot options and kernel-hashes=on for the confidential guest measured boot:

```sh
$ qemu-system-x86_64 \
  -enable-kvm \
  -machine q35 \
  -cpu EPYC-v4 \
  -machine memory-encryption=sev0 \
  -monitor none \
  -display none \
  -object memory-backend-memfd,id=ram1,size=2048M \
  -machine memory-backend=ram1 \
  -object sev-snp-guest,id=sev0,cbitpos=51,reduced-phys-bits=1,kernel-hashes=on" \
  -bios ${OVMF_PATH} 
  -kernel ${EFI_PATH}
```

- `$EFI_PATH`: 
  - If you're running inside a host image from this repository, the guest image is embedded at: `/usr/local/lib/guest-image/guest.efi`.
  - Otherwise, set this to the path of the guest image downloaded/built in step 1.

- `$OVMF_PATH`: either `/usr/share/ovmf/OVMF.amdsev.fd` or `/usr/share/edk2/ovmf/OVMF.amdsev.fd`, depending on your distro.



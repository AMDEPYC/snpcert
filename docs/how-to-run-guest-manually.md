The typical way to run tests are fully automated. However if you're developing tests or isolating an issue, you may find it useful to run guests manually. The following instructions assume you are running on AMD EPYC hardware that has been fully enabled for SEV.

# Prerequisites

Ensure that you are running on an AMD EPYC hardware & kernel combination that is fully enabled for SEV 3.0 (SNP). See the [AMD SEV Tuning Guide](https://www.amd.com/content/dam/amd/en/documents/epyc-technical-docs/tuning-guides/58207-using-sev-with-amd-epyc-processors.pdf) for supported kernel levels and firmware enablement instructions.

You can verify this by running the following command and resolving any failures:

```
snphost ok
```

If you are using a host image built by this repository, snphost is already installed in /usr/local/bin/snphost. If you are not, you will need to download it here: https://github.com/virtee/snphost

# How to Run

1. Download the build artifacts for your relevant guest distro.

2. Unzip the artifacts.

3. <ins>**Launch SNP Guest:** </ins>   Run an SNP guest with the direct boot options and kernel-hashes=on for the confidential guest measured boot:

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

Where `$OVMF_PATH` is either `/usr/share/ovmf/OVMF.amdsev.fd` or `/usr/share/edk2/ovmf/OVMF.amdsev.fd`, depending on your distro.


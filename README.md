# Welcome to SEV-SNP OS Certification

This contains images constructed using `mkosi`. The specific images all
inherit from the `common` image data. This means that we can put things
like `snpguest` tests and other workloads in `common` to be shared by
all of the OS images.

Individual guest tests should be written as systemd services.

# How to Run

1. Download the build artifacts for your relevant guest distro.

2. Unzip the artifacts.

3. Run them:

```sh
$ qemu-kvm -m 2G \
    -bios /usr/share/edk2/ovmf/OVMF_CODE.fd \
    -kernel guest-fedora-41/image.efi \
    -hda guest-fedora-41/image.qcow2
```

4. <ins>**Launch SNP Guest:** </ins>   Run an SNP guest with the direct boot options and kernel-hashes=on for the confidential guest measured boot:

```sh
$ qemu-system-x86_64 \
    -enable-kvm \
    -cpu EPYC-v4 \
    -smp 1 \
    -device virtio-blk-pci,drive=disk0,id=scsi0 \
    -drive file=guest-fedora-41/image.qcow2,if=none,id=disk0 \
    -machine memory-encryption=sev0,vmport=off \
    -object memory-backend-memfd,id=ram1,size=2048M \
    -machine memory-backend=ram1 \
    -object sev-snp-guest,id=sev0,cbitpos=51,reduced-phys-bits=1,kernel-hashes=on \
    -bios /usr/share/edk2/ovmf/OVMF.amdsev.fd \
    -kernel guest-fedora-41/image.efi \
    -nographic
```

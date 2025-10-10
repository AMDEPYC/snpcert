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


4. <ins>**Launch SNP Guest:** </ins>   Run an SNP guest with the direct boot options and kernel-hashes=on for the confidential guest measured boot:

```sh
$ qemu-system-x86_64 \
    -enable-kvm \
    -cpu EPYC-v4 \
    -smp 1 \
    -device virtio-blk-pci,drive=disk0,id=scsi0 \
    -drive file=guest/fedora/41/image.qcow2,if=none,id=disk0 \
    -machine memory-encryption=sev0,vmport=off \
    -object memory-backend-memfd,id=ram1,size=2048M \
    -machine memory-backend=ram1 \
    -object sev-snp-guest,id=sev0,cbitpos=51,reduced-phys-bits=1,kernel-hashes=on \
    -bios /usr/share/edk2/ovmf/OVMF.amdsev.fd \
    -kernel guest/fedora/41/image.efi \
    -nographic
```

5. <ins>**Share host files with the SNP guest:** </ins>
   a)  Create a host directory to expose certain host files to the SNP guest:

   ```
    $ mkdir shared_directory
   ```

   b) Add some required host files to be shared with the guest inside the `shared_directory`:

   ```
    $ cp /usr/share/edk2/ovmf/OVMF.amdsev.fd shared_directory
    $ cp guest/fedora/41/image.efi shared_directory
   ```


    c) Re-start the same SNP guest with the additional VirtFS qemu option to enable the access of the host files by SNP Guest using the 9P network protocol :

    ```sh
    $ qemu-system-x86_64 \
        -enable-kvm \
        -cpu EPYC-v4 \
        -smp 1 \
        -device virtio-blk-pci,drive=disk0,id=scsi0 \
        -drive file=guest/fedora/41/image.qcow2,if=none,id=disk0 \
        -machine memory-encryption=sev0,vmport=off \
        -object memory-backend-memfd,id=ram1,size=2048M \
        -machine memory-backend=ram1 \
        -object sev-snp-guest,id=sev0,cbitpos=51,reduced-phys-bits=1,kernel-hashes=on \
        -bios /usr/share/edk2/ovmf/OVMF.amdsev.fd \
        -kernel guest/fedora/41/image.efi \
        -nographic \
        -virtfs local,path=shared_directory,mount_tag,mount_tag=shared,security_model=mapped,id=fs0
    ```

    d)**Mounting the shared host path:**

    i) Create a mount point path inside the SNP Guest

    ```
        $ mkdir /etc/shared_host_directory
    ```

    ii) Mount the shared host folder with the `shared` mount tag using virtio 9P transport method :

    ```
        $  mount -t 9p -o trans=virtio shared /etc/shared_host_directory
    ```

# Welcome to SEV-SNP OS Certification

This contains images constructed using `mkosi`. The specific images all
inherit from the `common` image data. This means that we can put things
like `snpguest` tests and other workloads in `common` to be shared by
all of the OS images.

Individual guest tests should be written as systemd services.


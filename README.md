# Welcome to SEV OS Certification


The purpose of this repository is to provide a unified framework for testing and certifying operating system support for [AMD Secure Encrypted Virtualization (SEV)](https://www.amd.com/en/developer/sev.html) features. Self-service tools are provided to run a series of certification tests using an AMD EPYC server, allowing for any user/organization to verify SEV support on a particular OS. 

**Note**: Currently only linux distributions supported by [`mkosi`](https://github.com/systemd/mkosi) are compatible with this framework.

## Certification Matrix

This table contains operating systems that have undergone certification testing for AMD features through this repository. 

| OS |  Status |  Certification Level |
|---|---|---|
| Ubuntu Plucky |  ✅ |  [v3.0-0](https://github.com/AMDEPYC/sev-certify/issues/154) |
| Debian Trixie |  ❌ |  [N/A](https://github.com/AMDEPYC/sev-certify/issues/152) |
| Fedora 41 |  ✅ |  [v3.0-0](https://github.com/AMDEPYC/sev-certify/issues/153) |
| CentOS 10 |  ✅ |  [v3.0-0](https://github.com/AMDEPYC/sev-certify/issues/151) |
| Rocky 10 |  ❌ |  N/A |

✅ Passing tests for latest certification level
❌ Not Certified for latest level

## Self-Service Certification Tools


Users/Organizations may target their own SEV-enabled EPYC server for self-service certification runs. Follow our guide on running an automated certification test [here](https://github.com/AMDEPYC/sev-certify/blob/update-readme/docs/how-to-generate-certs.md).

## Certification Result Information

Each certification run automatically creates a GitHub Issue in this repository, documenting the results and assigning a certification level. Issues are tagged by OS and SEV feature to facilitate searching and tracking.

_Issue tags and details to be added here._

## Images


Host and Guest images are constructed in GitHub Workflows via [`mkosi`](https://github.com/systemd/mkosi). Host images are designed to be booted on a SEV-enabled EPYC server, and are configured with a series of custom systemd services that will run tests on an embedded guest image. The resulting host and guest images are available in GitHub releases in this repository.


## Project Organization

The repository is organized as follows:

- `docs/`: Documentation, guides, and images for using and understanding the project.
- `images/`: Host and guest OS image build configurations for various distributions.
- `modules/`: Modular components, scripts, and systemd service definitions for building and running certification workflows. Key submodules include:
	- `build/`: General build configuration for the modular system.
	- `common/`: Shared configuration files for use by multiple modules.
	- `embed-guest-image/`: Scripts and configs for embedding guest images into host images.
	- `guest/`: Guest OS configuration for certification.
	- `guest-measurement/`: Tools and scripts for generating the expected measurement for the test guest.
	- `guest-notices/`: Systemd configuration and notification handling for guests.
	- `host/`: Host OS configuration for certification.
	- `launch-snp-guest/`: Scripts for launching SEV-SNP guests.
	- `load-kernel-modules/`: Build scripts for loading required kernel modules.
	- `logging/`: Logging utilities, including:
		- `capture-guest-logs-in-host/`: Capture guest logs from the host.
		- `display-guest-environment/`: Display guest environment details.
		- `display-guest-logs/`: Display guest logs.
		- `sev-certificate-generator/`: Tools for generating SEV certificates.
		- `upload-guest-logs/`: Utilities for uploading guest logs.
	- `reboot/`: Scripts for reboot operations during workflows.
	- `snpguest/`: Embedded guest tests and attestation tools.
	- `snpguest-build/`: Build scripts for SEV-SNP guest images.
	- `snphost/`: Build scripts and configuration for SEV-SNP host images.


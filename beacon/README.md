# beacon

**`dispatch` workload communication client**

`beacon` is the companion client to `dispatch` that runs inside EFI workloads.
It automatically discovers `dispatch` services on the network and communicates
workload execution status and test results back to the orchestrator, enabling
automated GitHub issue creation for test reporting.

## Quick Start

```bash
# Signal workload boot
beacon boot

# Report test results
beacon report --title "Test Results" --body results.txt
```

## Core Features

- **Service Discovery**: Automatically finds `dispatch` servers via mDNS
- **Boot Notification**: Signals workload start to orchestrator
- **Result Reporting**: Submits test results for GitHub issue creation

## Commands

### `beacon boot`

Notifies `dispatch` that the workload has started execution:

```bash
beacon boot
```

This moves the workload from 📥 **Downloading** to ⚡ **Booting** state.

### `beacon report`

Submits test results to `dispatch` for GitHub issue creation:

```bash
# Report with body from file
beacon report --title "Test Results" --body results.txt

# Report with body from stdin
echo "All tests passed!" | beacon report --title "Success" --label bug --assignee npmccallum
```

## Exit Codes

- `0`: Command succeeded (connected to dispatch)
- `1`: No dispatch service found or connection failed

## Minimum Supported Rust Version (MSRV)

Rust 1.70+

---

**License**: MIT  
**Repository**: https://github.com/AMDEPYC/snpcert

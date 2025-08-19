# GitHub Download Proxy

[![CI](https://github.com/AMDEPYC/snpcert/actions/workflows/ghdp-ci.yml/badge.svg)](https://github.com/AMDEPYC/snpcert/actions/workflows/ghdp-ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.82-blue)](https://github.com/rust-lang/rust/releases/tag/1.82.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A tiny HTTP proxy that serves stable, redirect-free URLs for GitHub release assets.

## Quick start

```bash
# Run a local proxy for a single repo
cargo run -- -o <owner> -r <repo>

# Then fetch a release asset without following redirects
curl http://127.0.0.1:8080/<tag-or-release>/<asset-name> -o asset
```

See `ghdp --help` for all flags.

## Why is this secure?

- **single-repo scope**: We lock to one `<owner>/<repo>`.
- **scoped redirects**: We only redirect to allowed domains (default: `githubusercontent.com`).
- **limited redirects**: We limit the number of redirects (default: 2).
- **pass-through**: We return the entire final response. There is no content filtering.
- **ca trust**: We use system CA trust via `reqwest`.

## Run as a systemd service

```ini
# /etc/systemd/system/ghdp.service
[Unit]
Description=GitHub Download Proxy (ghdp)
After=network-online.target

[Service]
ExecStart=/usr/local/bin/ghdp -o <owner> -r <repo> -b 0.0.0.0:8080
Environment=RUST_LOG=info
Restart=on-failure
Group=nobody
User=nobody

[Install]
WantedBy=multi-user.target
```

Then:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now ghdp
```

## License

MIT. See `LICENSE`.

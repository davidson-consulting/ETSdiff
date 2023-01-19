<!--
SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
SPDX-License-Identifier: CC-BY-NC-SA-4.0
-->

[![REUSE status](https://api.reuse.software/badge/github.com/fsfe/reuse-tool)](https://api.reuse.software/info/github.com/fsfe/reuse-tool)

# ETSdiff

Analyses consumption of a program with 3 criteria: energy, transfer and storage 

## Prerequisite

*Temporary process before the v1.0 that provide a deb package*

**ETSdiff and dependencies need to works with privileged access -> so you need to be in sudoer list**

1. Install [vjoule](https://github.com/davidson-consulting/vjoule/releases/tag/v0.2) from deb package
    * update `/etc/vjoule/sensor/config.toml` by adding `"controlled.slice"` into `slices`
1. Ubuntu depenencies:
    * `sudo apt install tshark`
1. Install [rust language](https://www.rust-lang.org/tools/install) 


## Getting started
Hey, from now it's simple as Rust so 
```
cargo build
cargo run
```

## License

This work is licensed under multiple licences. Because keeping this section
up-to-date is challenging, here is a brief summary:

- All original source code is licensed under GPL-3.0-or-later.
- All documentation is licensed under CC-BY-SA-4.0.
- Some configuration and data files are licensed under CC0-1.0.

For more accurate information, check the individual files.

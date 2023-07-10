<!--
SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
SPDX-License-Identifier: CC-BY-NC-SA-4.0
-->

# ETSdiff: WebClientService

**Development stage**

Here you can find a python script `webclientservice.py` than can run a firefox and replay a test scenario from [selenium](/home/vincent/dev/dav-github/ETSdiff-webclient/webclient-service/) `side` file.

To ensure right measurement when you record a scenario add description `BeforeTEST` and `AfterTEST` manualy by the Selenium IDE.

## Runing

On the `test` directory you can find all the necessary file for a basic usage.

1. From `webclient-service` directory run the service as root user `python3 webclientservice.py ./test/impakt.side /tmp/wcs`
1. From `ETSdiff` root directory run `cargo run $PWD/webclient-service/test/impakt.toml`


_in the future the webclient service must be embeded in main rust program_

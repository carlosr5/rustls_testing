# rustls_testing
This is for my Summer 2023 Intern Project at Microsoft. This project is for us to create a benchmark test for RusTLS (rustls), a Rust-based TLS client/server codebase using webpki certificate validation, and compare that against using SChannel and other platform-native certificate validation libraries (through rustls-platform-verifier for now).

## How to Use
After you get the repo on your machine and you're in the directory, you can set environment variables to test certain configurations. For instance, you can do
`$env:ARG_HOSTNAME="mozilla-modern.badssl.com"; $env:HTTP="true"; cargo bench` to run the benchmark tests where the client tries to connect to `mozilla-modern.badssl.com`. Test results are visible in `./target/criterion/client_rtt` where the tests tries a connection 100 times by default.

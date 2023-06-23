# rustls_testing
This is for my Summer 2023 Intern Project at Microsoft. This project is for us to create a benchmark test for RusTLS (rustls), a Rust-based TLS client/server codebase using webpki certificate validation, and compare that against using SChannel and other platform-native certificate validation libraries. We first compare against rustls-platform-verifier, a Rust library that has bindings to the OS's cert validation libraries. In the next few weeks I plan to add test support for rust-native-tls as well flesh out this project in the next 6 weeks.

## How to Use
After you get the repo on your machine and you're in the directory, you can set environment variables to test certain configurations. For instance, you can do
`$env:ARG_HOSTNAME="mozilla-modern.badssl.com"; $env:HTTP="true"; cargo bench` to run the benchmark tests where the client tries to connect to `mozilla-modern.badssl.com`. Test results are visible in `./target/criterion/client_rtt` where the tests tries a connection 100 times by default.

## Testing
I use [Critieron](https://bheisler.github.io/criterion.rs/book/index.html) for benchmarking TLS client connections. It's a relatively light-weight benchmarking framework that provides a solid statistical analysis of any given function and its runtime. Criterion provides confidence integral ranges for the average runtime, notes any statistically significant change (using a p-value of 0.05 by default), and provides all this information neatly summarized in an index.html file (with the raw data available if needed). 

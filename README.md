Hello there! Welcome to my Microsoft Summer 2023 Intern Project! This README serves as the documentation for this project, outlining why this project exists, how to run it, as well as what to look at in the future.

# Project Motivation

Microsoft is considering doing future systems programming work in Rust, as Rust offers more security and eliminates whole classes of bugs that other systems programming languages like C/C++ come with. In order for us to transition, we need to expand Rust support in multiple security areas, such as with TLS connections. This project is meant to see how different Rust TLS implementations / configurations perform against one another. This will help inform whether we can move over to Rust while still keeping our performant programs.

This project assumes background knowledge in networking (the TLS protocol and certificate validation), as well as a basic understanding of Rust. For understanding the test results, a basic probability/statistics background is sufficient, but the testing library has stuff for those who got an A in their probability/statistics class.

## What’s Being Tested

The original goal of the project was to test the performance of two different certificate validation libraries: webpki, a light-weight, browser-focused certificate validation library, and rustls-platform-verifier, a library that calls the OS certificate verifier. Both of these would be used inside of rustTLS (rustls), a Rust TLS implementation. The rustls implementation that I use in here uses either webpki or platform-verifier as its main configuration.

I had additional time to test against rust-native-tls, a Rust wrapper that calls the OS TLS implementation (e.x. SChannel for Windows). This is a separate library from rustls.

## How we’re testing

I use [Critieron](https://bheisler.github.io/criterion.rs/book/index.html) for benchmarking TLS client connections. It's a relatively light-weight benchmarking framework that provides a solid statistical analysis of any given function and its runtime. Criterion provides confidence integral ranges for the average runtime, notes any statistically significant change (using a p-value of 0.05 by default), and provides all this information neatly summarized in an index.html file (with the raw data available if needed).

# Setup / How to Use

Make sure you have the latest stable version of Rust installed on your computer. We’ll first clone this repo, but then we’ll have to [download the rustls-platform-verifier repo](https://github.com/rustls/rustls-platform-verifier) and have our dependency path be updated inside of the Cargo.toml file on line 10 inside of rustls_testing. Platform-verifier isn’t available through Cargo yet, but once it is then we’ll be able to change our .toml file so it can be like any other dependency listed.

**Note: Set the environment variables in every terminal that you use in your testing.**

## HTTP Connections

If we want to run against an HTTP connection, we’ll use the same site as the rustls tests do and set all other environment variables to be empty. 

```powershell
# Run below for testing between a sample website and our client
$env:ARG_HOSTNAME="mozilla-modern.badssl.com";$env:HTTP="true";$env:VERBOSE="false";$env:PORT="";$env:PROTOVER="";$env:SUITE="";$env:PROTO="";$env:MAX_FRAG_SIZE="";$env:CAFILE="";$env:NO_TICKETS="";$env:NO_SNI="";$env:INSECURE="";$env:AUTH_KEY="";$env:AUTH_CERTS="";$env:CMD_ECHO="";$env:CMD_HTTP="";$env:WEBPKI_PORT="";$env:PLATFORM_PORT="";$env:CERTS="";$env:WEBPKI_CERTS="";$env:PLATFORM_CERTS="";$env:KEY="";$env:WEBPKI_KEY="";$env:PLATFORM_KEY="";$env:OCSP="";$env:AUTH="";$env:REQUIRE_AUTH="";$env:RESUMPTION="";$env:TICKETS="";$env:ARG_FPORT=""; cargo bench
```

## Self-Hosted Server Connections

If we want to run against a self-hosted server, then we’ll have to run the following environment variables in each server terminal that we use (in this case, one for webpki and one for platform-verifier), and also in our benchmarking terminal.

```powershell
# CAFILE arg is only consumed by default configuration, so we don't need additonal checking.
# Run below in each terminal used for testing between a locally-run server and our client
$env:WEBPKI_PORT="8443";$env:PLATFORM_PORT="8445";$env:ARG_HOSTNAME="localhost";$env:HTTP="false";$env:CAFILE="../rustls/test-ca/rsa/ca.cert";$env:WEBPKI_CERTS="../rustls/test-ca/rsa/end.fullchain";$env:PLATFORM_CERTS="./converted.pem";$env:WEBPKI_KEY="../rustls/test-ca/rsa/end.rsa";$env:PLATFORM_KEY="./privatekey.pem";$env:CMD_ECHO="true";$env:VERBOSE="false";$env:PORT="";$env:PROTOVER="";$env:SUITE="";$env:PROTO="";$env:MAX_FRAG_SIZE="";$env:NO_TICKETS="";$env:NO_SNI="";$env:INSECURE="";$env:AUTH_KEY="";$env:AUTH_CERTS="";$env:CMD_HTTP="";$env:CERTS="";$env:KEY="";$env:OCSP="";$env:AUTH="";$env:REQUIRE_AUTH="";$env:RESUMPTION="";$env:TICKETS="";$env:ARG_FPORT="";

# Run the following two commands in separate terminals
cargo run --bin tlsserver_mio -- --run_type webpki
cargo run --bin tlsserver_mio -- --run_type rustls-platform-verifier
cargo bench
```

// Talk about how to get roots trusted on a Windows machine

Note: Testing against this self-hosted server does not work for rust-native-tls at the moment and is a bug that should be fixed before further expanding this testing framework

# Areas of Improvement

There are some different ways we can improve this program:

## More Rigorous Testing

We can test against our self-hosted server where the server is at full load and then check to see how long a client takes to connect. This should make the timing between tests more consistent, and also doubles as a test to how robust the server is. We could even expand this to test against different server configurations in Rust (like a rustls server versus a rust-native-tls server).

We can also test this on different architecture types (ARM vs AMD, CISC, vs RISC, etc). We may be able to find some performance differences between the different tests if we do this.

## Better Program Interface

Entering in all of the environment variables is frustrating! Having a wrapper program where we can pass in the flags we want in a single command would make this a lot easier to work with. From what I’ve found online as of July 2023, this seems to be more of a Rust benchmarking issue more than a Criterion issue.

# Next Steps

As of right now regarding Microsoft’s move towards Rust, it’s gradual. Having more robust testing will be good to inform our decisions and what we choose to spend effort on, but this is still a relatively new direction that Microsoft is moving in. I personally expect / would like for this test to be fleshed out a lot more so we can have enough information to know what we’re getting ourselves into. Otherwise, our team is making other moves towards moving to Rust, namely collaborating with the rustls maintainers to have Microsoft be a contributor to the library.
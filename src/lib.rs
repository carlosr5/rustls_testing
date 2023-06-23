/// The following is utility code for benchmarking a client connection
mod tlsclient_mio;

use tlsclient_mio::{run_configured_client, run_platform_verifier_client};
use serde::Deserialize;

/// Struct to hold the arguments passed in from the command line..
#[derive(Deserialize, Debug)]
pub struct BenchArgs {
    port: Option<u16>,
    http: Option<bool>,
    verbose: Option<bool>,
    protover: Option<Vec<String>>,
    suite: Option<Vec<String>>,
    proto: Option<Vec<String>>,
    max_frag_size: Option<usize>,
    cafile: Option<String>,
    no_tickets: Option<bool>,
    no_sni: Option<bool>,
    insecure: Option<bool>,
    auth_key: Option<String>,
    auth_certs: Option<String>,
    arg_hostname: String,
}

impl BenchArgs {
    /// Returns a new instance of BenchArgs with default values set.
    pub fn with_defaults(self) -> Self {
        Self {
            port: Some(self.port.unwrap_or(443)),
            http: Some(self.http.unwrap_or(false)),
            verbose: Some(self.verbose.unwrap_or(false)),
            protover: Some(self.protover.unwrap_or(Vec::new())),
            suite: Some(self.suite.unwrap_or(Vec::new())),
            proto: Some(self.proto.unwrap_or(Vec::new())),
            // Setting an arbitrary default value for max_frag_size for now
            max_frag_size: Some(self.max_frag_size.unwrap_or(12345)),
            cafile: self.cafile,  // keeping as is, not sure if a default is required
            no_tickets: Some(self.no_tickets.unwrap_or(false)),
            no_sni: Some(self.no_sni.unwrap_or(false)),
            insecure: Some(self.insecure.unwrap_or(false)),
            auth_key: self.auth_key,  // keeping as is, not sure if a default is required
            auth_certs: self.auth_certs,  // keeping as is, not sure if a default is required
            arg_hostname: self.arg_hostname,
        }
    }
}

/// Starts a TLS client connection to a server. 
/// Takes in a list of arguments to configure the connection.
/// We need this function in here so that Criterion can find and test this function.
#[inline]
pub fn start_client() {
    // First, we'll grab all of the environment variables that we want from the terminal.
    // We'll let the envy crate grab everything for us, and then we'll add some default values.
    let args: BenchArgs = envy::from_env::<BenchArgs>().unwrap().with_defaults();

    // Then, we'll pass in the arguments to the tlsclient_mio file to start the client.
    run_configured_client(args);
}

#[inline]
pub fn start_platform_verifier_client() {
    // First, we'll grab all of the environment variables that we want from the terminal.
    // We'll let the envy crate grab everything for us, and then we'll add some default values.
    let args: BenchArgs = envy::from_env::<BenchArgs>().unwrap().with_defaults();

    // Then, we'll pass in the arguments to the tlsclient_mio file to start the client.
    run_platform_verifier_client(args);
}

// /// Starts a TLS server that waits for client connections.
// /// Takes in a list of arguments to configure the connection
// /// At the moment, this function isn't implemented yet, but we'll need it for Criterion to be able to run the server from the benchmarking tests.
// #[inline]
// pub fn start_server(args: Vec<String>) {

// }
/// The following is utility code for benchmarking a client connection
mod tlsclient_mio;

use tlsclient_mio::run_configured_client;

// mod tlsserver_mio;

// use tlsserver_mio::run_configured_server;

use serde::Deserialize;

/// Struct to hold the arguments passed in from the command line.
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
pub fn start_client(cert_type: String) {
    // First, we'll grab all of the environment variables that we want from the terminal.
    // We'll let the envy crate grab everything for us, and then we'll add some default values.
    let args: BenchArgs = envy::from_env::<BenchArgs>().unwrap().with_defaults();

    //     // // Print every element inside the args struct
//     // println!("args.port: {:?}", args.port);
//     // println!("args.http: {:?}", args.http);
//     // println!("args.verbose: {:?}", args.verbose);
//     // println!("args.protover: {:?}", args.protover);
//     // println!("args.suite: {:?}", args.suite);
//     // println!("args.proto: {:?}", args.proto);
//     // println!("args.max_frag_size: {:?}", args.max_frag_size);
//     // println!("args.cafile: {:?}", args.cafile);
//     // println!("args.no_tickets: {:?}", args.no_tickets);
//     // println!("args.no_sni: {:?}", args.no_sni);
//     // println!("args.insecure: {:?}", args.insecure);
//     // println!("args.auth_key: {:?}", args.auth_key);
//     // println!("args.auth_certs: {:?}", args.auth_certs);
//     // println!("args.arg_hostname: {:?}", args.arg_hostname);


    // Then, we'll pass in the arguments to the tlsclient_mio file to start the client.
    run_configured_client(args, cert_type);
}

/// Starts a TLS server that waits for client connections.
/// Takes in a list of arguments to configure the connection
/// At the moment, this function isn't implemented yet, but we'll need it for Criterion to be able to run the server from the benchmarking tests.
#[inline]
pub fn start_server() {
    let args: BenchArgs = envy::from_env::<BenchArgs>().unwrap().with_defaults();

    // run_configured_server(args);
}
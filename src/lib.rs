/// The following is utility code for benchmarking a client connection
mod tlsclient_mio;

use tlsclient_mio::run_configured_client;

use std::net::TcpStream;
use native_tls::TlsConnector;

use std::io::Write;
use serde::Deserialize;

/// Struct to hold the arguments passed in from the command line.
#[derive(Deserialize, Debug)]
pub struct BenchArgs {
    pub port: Option<u16>,
    pub http: Option<bool>,
    pub verbose: Option<bool>,
    pub protover: Option<Vec<String>>,
    pub suite: Option<Vec<String>>,
    pub proto: Option<Vec<String>>,
    // Client args
    pub max_frag_size: Option<usize>,
    pub cafile: Option<String>,
    pub no_tickets: Option<bool>,
    pub no_sni: Option<bool>,
    pub insecure: Option<bool>,
    pub auth_key: Option<String>,
    pub auth_certs: Option<String>,
    pub arg_hostname: String,
    // Server args
    pub cmd_echo: Option<bool>,
    pub cmd_http: Option<bool>,
    pub webpki_port: Option<u16>,
    pub platform_port: Option<u16>,
    pub certs: Option<String>,
    pub webpki_certs: Option<String>,
    pub platform_certs: Option<String>,
    pub key: Option<String>,
    pub webpki_key: Option<String>,
    pub platform_key: Option<String>,
    pub ocsp: Option<String>,
    pub auth: Option<String>,
    pub require_auth: Option<bool>,
    pub resumption: Option<bool>,
    pub tickets: Option<bool>,
    pub arg_fport: Option<u16>,
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
            // Client defauls
            max_frag_size: Some(self.max_frag_size.unwrap_or(12345)),
            cafile: self.cafile,  // keeping as is, not sure if a default is required
            no_tickets: Some(self.no_tickets.unwrap_or(false)),
            no_sni: Some(self.no_sni.unwrap_or(false)),
            insecure: Some(self.insecure.unwrap_or(false)),
            auth_key: self.auth_key,  // keeping as is, not sure if a default is required
            auth_certs: self.auth_certs,  // keeping as is, not sure if a default is required
            arg_hostname: self.arg_hostname,
            // Server defaults
            cmd_echo: Some(self.cmd_echo.unwrap_or(false)),
            cmd_http: Some(self.cmd_http.unwrap_or(false)),
            webpki_port: Some(self.webpki_port.unwrap_or(443)),
            platform_port: Some(self.platform_port.unwrap_or(443)),
            certs: self.certs,
            webpki_certs: self.webpki_certs,
            platform_certs: self.platform_certs,
            key: self.key,
            webpki_key: self.webpki_key,
            platform_key: self.platform_key,
            ocsp: self.ocsp,
            auth: self.auth,
            require_auth: Some(self.require_auth.unwrap_or(false)),
            resumption: Some(self.resumption.unwrap_or(false)),
            tickets: Some(self.tickets.unwrap_or(false)),
            arg_fport: self.arg_fport,
        }
    }

    pub fn print(&self) {
        println!("BenchArgs {{
            port: {:?},
            http: {:?},
            verbose: {:?},
            protover: {:?},
            suite: {:?},
            proto: {:?},
            max_frag_size: {:?},
            cafile: {:?},
            no_tickets: {:?},
            no_sni: {:?},
            insecure: {:?},
            auth_key: {:?},
            auth_certs: {:?},
            arg_hostname: {:?},
            cmd_echo: {:?},
            cmd_http: {:?},
            webpki_port: {:?},
            platform_port: {:?},
            certs: {:?},
            webpki_certs: {:?},
            platform_certs: {:?},
            key: {:?},
            webpki_key: {:?},
            platform_key: {:?},
            ocsp: {:?},
            auth: {:?},
            require_auth: {:?},
            resumption: {:?},
            tickets: {:?},
            arg_fport: {:?}
        }}",
            self.port,
            self.http,
            self.verbose,
            self.protover.as_ref().map(|v| v.join(", ")),
            self.suite.as_ref().map(|v| v.join(", ")),
            self.proto.as_ref().map(|v| v.join(", ")),
            self.max_frag_size,
            self.cafile,
            self.no_tickets,
            self.no_sni,
            self.insecure,
            self.auth_key,
            self.auth_certs,
            self.arg_hostname,
            self.cmd_echo,
            self.cmd_http,
            self.webpki_port,
            self.platform_port,
            self.certs,
            self.webpki_certs,
            self.platform_certs,
            self.key,
            self.webpki_key,
            self.platform_key,
            self.ocsp,
            self.auth,
            self.require_auth,
            self.resumption,
            self.tickets,
            self.arg_fport
        );
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

    // args.print();
    if cert_type.eq_ignore_ascii_case("rust-native-tls") {
        // println!("Inside rust-native-tls branch");
        let connector = TlsConnector::new().unwrap();

        let hostname = "mozilla-modern.badssl.com".to_string();
        let port = "443".to_string();
        // let hostname = args.arg_hostname;
        // let port = args.platform_port.unwrap();
        let addr = hostname.clone() + ":" + port.to_string().as_str();

        let stream = TcpStream::connect(addr).unwrap();
        
        let mut stream = connector.connect(hostname.clone().as_str(), stream).unwrap();

        
        stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    }
    else {
         // Then, we'll pass in the arguments to the tlsclient_mio file to start the client.
        run_configured_client(args, cert_type);
    }
}

// /// Starts a TLS server that waits for client connections.
// /// Takes in a list of arguments to configure the connection
// /// At the moment, this function isn't implemented yet, but we'll need it for Criterion to be able to run the server from the benchmarking tests.
// #[inline]
// pub fn start_server(run_type: &str) {
//     let args: BenchArgs = envy::from_env::<BenchArgs>().unwrap().with_defaults();

//     run_configured_server(args, run_type);
// }
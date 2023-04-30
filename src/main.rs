use std::default;

use rsip::prelude::*;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut connection = TcpStream::connect("192.168.0.109:5060");
    let request = generate_register_request("192.168.0.1005:4000", "Hiren", "Bob", "192.168.0.1005:5000");

    match connection{
        //TODO: Convert struct to grpc format or Serde?
        Ok(conn) => println!("Connection succeeded"),
        Err(e)=>println!("Couldn't connect to 192.168.0.109:5060\n {:?}",e)
    }
    println!("{}", request);
}
fn generate_register_request(host: &str, from: &str, to: &str, client:&str) -> rsip::SipMessage {
    let mut headers: rsip::Headers = Default::default();

    let base_uri = rsip::Uri {
        scheme: Some(rsip::Scheme::Sip),
        host_with_port: rsip::Domain::from(host).into(),
        ..Default::default()
    };

    headers.push(
        rsip::typed::Via {
            version: rsip::Version::V2,
            transport: rsip::Transport::Tls,
            uri: rsip::Uri {
                host_with_port: (rsip::Domain::from(client), 5060).into(),
                ..Default::default()
            },
            params: vec![rsip::Param::Branch(rsip::param::Branch::new(
                "z9hG4bKnashds7",
            ))],
        }
        .into(),
    );
    headers.push(rsip::headers::MaxForwards::default().into());
    headers.push(
        rsip::typed::From {
            display_name: Some(from.into()),
            uri: base_uri.clone(),
            params: vec![rsip::Param::Tag(rsip::param::Tag::new("a73kszlfl"))],
        }
        .into(),
    );
    headers.push(
        rsip::typed::To {
            display_name: Some(to.into()),
            uri: base_uri.clone(),
            params: Default::default(),
        }
        .into(),
    );
    headers.push(rsip::headers::CallId::default().into());
    headers.push(
        rsip::typed::CSeq {
            seq: 1,
            method: rsip::Method::Register,
        }
        .into(),
    );
    headers.push(
        rsip::typed::Contact {
            display_name: None,
            uri: base_uri,
            params: Default::default(),
        }
        .into(),
    );
    headers.push(rsip::headers::ContentLength::default().into());

    rsip::Request {
        method: rsip::Method::Register,
        uri: rsip::Uri {
            scheme: Some(rsip::Scheme::Sips),
            host_with_port: rsip::Domain::from(host).into(),
            ..Default::default()
        },
        version: rsip::Version::V2,
        headers: headers,
        body: Default::default(),
    }
    .into()
}

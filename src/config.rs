use crate::{backend::*, constants::*, globals::*};
use hyper::Uri;
use std::{collections::HashMap, sync::Mutex};

// #[cfg(feature = "tls")]
use std::path::PathBuf;

pub fn parse_opts(globals: &mut Globals, backends: &mut HashMap<String, Backend>) {
  // TODO:
  globals.listen_sockets = LISTEN_ADDRESSES
    .to_vec()
    .iter()
    .flat_map(|x| {
      vec![
        format!("{}:{}", x, HTTP_LISTEN_PORT).parse().unwrap(),
        format!("{}:{}", x, HTTPS_LISTEN_PORT).parse().unwrap(),
      ]
    })
    .collect();
  globals.http_port = Some(HTTP_LISTEN_PORT);
  globals.https_port = Some(HTTPS_LISTEN_PORT);

  // TODO:
  let mut map_example: HashMap<String, Uri> = HashMap::new();
  map_example.insert(
    "/maps".to_string(),
    "https://bing.com/".parse::<Uri>().unwrap(),
  );
  backends.insert(
    "localhost".to_string(),
    Backend {
      app_name: "Google except for maps".to_string(),
      hostname: "google.com".to_string(),
      reverse_proxy: ReverseProxy {
        default_destination_uri: "https://google.com/".parse::<Uri>().unwrap(),
        destination_uris: Some(map_example),
      },
      redirect_to_https: None, // TODO: ここはHTTPの時のみの設定。tlsの存在とは排他的。

      tls_cert_path: Some(PathBuf::from(r"localhost1.pem")),
      tls_cert_key_path: Some(PathBuf::from(r"localhost1.pem")),
      server_config: Mutex::new(None),
    },
  );
}

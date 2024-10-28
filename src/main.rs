use std::env;
use std::net::{ Ipv4Addr, SocketAddr, SocketAddrV4 };
use std::str::FromStr;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::time::Instant;

use config::init_config;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use fileserver::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let config_arc = Arc::new(init_config()?);
    log::info!("config loaded: {config_arc:?}");

    let start = Instant::now();
    filesystem::generate_file_list_html(&config_arc.fs_dir).await?;
    let file_list_dur = start.elapsed();
    log::debug!("generating file-list.html took {file_list_dur:?}");

    let ip = match Ipv4Addr::from_str(&config_arc.ipv4_addr) {
        Ok(ip) => ip,
        Err(err) => {
            log::warn!("failed to parse ip ({err}), switching to 0.0.0.0");
            Ipv4Addr::UNSPECIFIED
        }
    };

    let addr: SocketAddr = SocketAddrV4::new(ip, config_arc.port).into();

    let listener = TcpListener::bind(addr).await?;
    log::info!("Listening on http://{}", addr);

    let mut conn_counter = 0u64;

    let active_connections = Arc::new(AtomicU64::new(0));

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let config = config_arc.clone();

        let active_connections = active_connections.clone();
        let id = conn_counter;

        tokio::task::spawn(async move {
            active_connections.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            log::trace!("> [{id}]");

            if
                let Err(err) = http1::Builder::new().serve_connection(
                    io,
                    service_fn(|req| {
                        let config = config.clone();
                        async move { api::handle_request(req, id, config).await }
                    })
                ).await
            {
                log::warn!("[{id}] Failed to serve connection: {:?}", err);
            }

            active_connections.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            log::trace!("< [{id}]");
        });

        conn_counter += 1;
    }
}

use std::env;
use std::net::{ Ipv4Addr, SocketAddr, SocketAddrV4 };
use std::sync::Arc;
use std::time::Instant;

use app_fs::{ filesystem, init_fs };
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use fileserver::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config_arc = Arc::new(init_fs()?);
    log::info!("config loaded: {config_arc:?}");

    let start = Instant::now();
    filesystem::generate_file_list_html(&config_arc.fs_dir).await?;
    let file_list_dur = start.elapsed();
    log::debug!("generating file-list.html took {file_list_dur:?}");

    let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config_arc.port).into();

    let listener = TcpListener::bind(addr).await?;
    log::info!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let config = config_arc.clone();

        tokio::task::spawn(async move {
            if
                let Err(err) = http1::Builder::new().serve_connection(
                    io,
                    service_fn(|req| {
                        let config = config.clone();
                        async move { api::handle_request(req, config).await }
                    })
                ).await
            {
                log::warn!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

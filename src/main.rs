use std::net::SocketAddr;
use std::time::Instant;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use hyper_api_exp::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let start = Instant::now();
    fs_tools::generate_file_list_html().await?;
    let file_list_dur = start.elapsed();
    println!("generating file-list.html took {file_list_dur:?}");

    let addr: SocketAddr = "0.0.0.0:1336".parse().unwrap();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if
                let Err(err) = http1::Builder
                    ::new()
                    .serve_connection(io, service_fn(api::handle_request)).await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

use std::path::PathBuf;

use bytes::Bytes;
use futures_util::TryStreamExt;
use http_body_util::{ combinators::BoxBody, BodyExt, Full, StreamBody };
use hyper::body::Frame;
use hyper::{ Method, Request, Response, Result, StatusCode };
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[allow(unused_imports)]
use crate::*;

pub async fn handle_request(
    req: Request<hyper::body::Incoming>
) -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    match (req.method(), req.uri().path(), req.uri().query()) {
        (&Method::GET, "/", _) => Ok(simple_response(StatusCode::OK, "try /open".to_string())),
        (&Method::GET, "/open", Some(mut query)) => {
            if query.contains("../") {
                Ok(simple_response(StatusCode::OK, "path can't contain ../".to_string()))
            } else {
                // remove leading """ because that would interfere with 'PathBuf::join(...)'
                while query.starts_with('/') {
                    query = query.strip_prefix('/').unwrap_or("");
                }

                let path = PathBuf::from(FS_DIR).join(query);

                println!("requested path: {}", path.display());

                simple_file_send(&path).await
            }
        }
        (&Method::GET, "/open", None) => simple_file_send(&PathBuf::from(FILE_LIST_PATH)).await,

        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<BoxBody<Bytes, std::io::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(
            Full::new("NOT FOUND".into())
                .map_err(|e| {
                    match e {
                    }
                })
                .boxed()
        )
        .unwrap()
}

fn simple_response(
    status_code: StatusCode,
    message: String
) -> Response<BoxBody<Bytes, std::io::Error>> {
    Response::builder()
        .status(status_code)
        .body(
            Full::new(message.into())
                .map_err(|e| {
                    match e {
                    }
                })
                .boxed()
        )
        .unwrap()
}

async fn simple_file_send(path: &PathBuf) -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    // Open file for reading
    let file = File::open(path).await;
    if file.is_err() {
        eprintln!("ERROR: Unable to open file.");
        return Ok(not_found());
    }

    let file: File = file.unwrap();

    // Wrap to a tokio_util::io::ReaderStream
    let reader_stream = ReaderStream::new(file);

    // Convert to http_body_util::BoxBody
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = stream_body.boxed();

    // Send response
    let response = Response::builder().status(StatusCode::OK).body(boxed_body).unwrap();

    Ok(response)
}

use std::path::PathBuf;

use bytes::Bytes;
use fs_tools::get_file_list;
use futures_util::TryStreamExt;
use http_body_util::{ combinators::BoxBody, BodyExt, Full, StreamBody };
use hyper::body::Frame;
use hyper::{ Method, Request, Response, Result, StatusCode };
use percent_encoding::percent_decode_str;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[allow(unused_imports)]
use crate::*;

pub const PAGE_NOT_FOUND: &'static str = include_str!("../assets/page-not-found.txt");

pub async fn handle_request(
    req: Request<hyper::body::Incoming>
) -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    let query = req
        .uri()
        .query()
        .map(|str| { percent_decode_str(str).decode_utf8_lossy().to_string() });

    match (req.method(), req.uri().path(), query) {
        // (&Method::GET, "/", _) => Ok(simple_response(StatusCode::OK, "try /file")),
        (&Method::GET, "/file", Some(mut query)) => {
            if query.contains("../") {
                Ok(simple_response(StatusCode::OK, "path can't contain ../"))
            } else {
                // remove leading """ because that would interfere with 'PathBuf::join(...)'
                while query.starts_with('/') {
                    query = query.strip_prefix('/').unwrap_or("").to_string();
                }

                let path = PathBuf::from(FS_DIR).join(query);

                println!("requested path: {}", path.display());

                simple_file_send(&path).await
            }
        }
        (&Method::GET, "/file-list.html", None) =>
            simple_file_send(&PathBuf::from(FILE_LIST_PATH)).await,
        (&Method::GET, "/file-list", query) => {
            let mut files = get_file_list().await;

            // add support for filtering queries (eg. /file-list?contains=...)
            // we simplify here for now: we take the entire query as pattern for the String::contains(pat) filter
            if let Some(query) = query {
                files = files
                    .into_iter()
                    .filter(|string| { string.contains(&query) })
                    .collect();
            }

            let mut files_string = String::new();
            for file in &files {
                files_string += file;
                files_string += "\n";
            }

            Ok(simple_response(StatusCode::OK, files_string))
        }

        _ => Ok(simple_response(StatusCode::NOT_FOUND, PAGE_NOT_FOUND)),
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

fn simple_response<B: Into<Bytes>>(
    status_code: StatusCode,
    message: B
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

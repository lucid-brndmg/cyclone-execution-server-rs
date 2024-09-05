mod config;
mod response;
mod cyclone;
mod environment;
mod execution;
mod parse;
mod cli;
mod handler;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use bytes::Bytes;
use http_body_util::{BodyExt, Full, Empty};
use hyper::{header, body::Incoming, Request, Response, StatusCode, Method};
use hyper::header::HeaderValue;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tracing::{debug, error, info, warn};
use hyper::server::conn::http1;
use serde::Serialize;
use tokio::net::TcpListener;
use crate::cli::CliArgs;
use crate::environment::{Environment};
use crate::handler::{handle_exec, handle_server_info};
use crate::response::{RESP_JSON_INTERNAL_ERROR};

const EDITOR_URL: &str = "https://cyclone.cs.nuim.ie/editor";
const REPO_URL: &str = "https://github.com/lucid-brndmg/cyclone-execution-server-rs";
// const ERROR_JSON: &str = concat!("{\"code\":", RESP_CODE_INTERNAL_ERROR, "}"); // format!("{{\"code\":{RESP_CODE_INTERNAL_ERROR}}}");

static NOTFOUND: &[u8] = b"Not Found";

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn empty() -> BoxBody {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn build_cors_resp(req: &Request<Incoming>) -> hyper::http::response::Builder {
    let hds = req.headers();
    let default_origin = HeaderValue::from_static("*");
    let default_req = HeaderValue::from_static("access-control-allow-origin,content-type");
    let hd_origin = hds.get(header::ORIGIN).unwrap_or(&default_origin);

    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, hd_origin)
        .header(header::ACCESS_CONTROL_ALLOW_METHODS, "GET,HEAD,PUT,POST,DELETE")
        .header(header::ACCESS_CONTROL_ALLOW_HEADERS, hds.get(header::ACCESS_CONTROL_REQUEST_HEADERS).unwrap_or(&default_req))
}

fn build_error_resp(builder: hyper::http::response::Builder) -> Response<BoxBody> {
    builder
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(RESP_JSON_INTERNAL_ERROR))
        .unwrap()
}

fn build_resp<T: ?Sized + Serialize>(builder: hyper::http::response::Builder, body: &T) -> Response<BoxBody> {
    match serde_json::to_string(body) {
        Ok(json) => builder
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(full(json))
            .unwrap(),
        Err(_) => build_error_resp(builder)
    }
}

async fn handle_api(req: Request<Incoming>, env: Arc<Environment>) -> Result<Response<BoxBody>> {
    let (method, path) = (req.method(), req.uri().path());
    debug!("Request: {} {}", method, path);
    match (method, path) {
        (&Method::GET, "/") => Ok(build_resp(build_cors_resp(&req), &handle_server_info(&env))),
        (&Method::POST, "/exec") => {
            let cors_resp_builder = build_cors_resp(&req);
            let req_body = req.collect().await?.to_bytes();
            let exec_req = serde_json::from_slice(&req_body);
            match exec_req {
                Ok(exec_req) => {
                    let result = handle_exec(&exec_req, &env).await;
                    Ok(build_resp(cors_resp_builder, &result))
                },
                Err(_) => Ok(build_error_resp(cors_resp_builder))
            }
        },
        (&Method::OPTIONS, _) => Ok(build_cors_resp(&req).status(StatusCode::NO_CONTENT).body(empty()).unwrap()),
        _ => Ok(Response::builder().status(StatusCode::NOT_FOUND).body(full(NOTFOUND)).unwrap())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("* Cyclone Online Editor's Execution Server *");
    println!("See {} for documents", REPO_URL);

    let args = CliArgs::init();
    let env = Environment::initialize(&args);

    let host = env.config.server.host.clone();
    let mut port = env.config.server.port;
    let mut error = None;
    let mut begin_range = false;
    let max_port = 9999;
    let min_port = 2000;

    let listener = loop {
        if begin_range && port > max_port {
            error!("Failed to find a port to start a server.");
            return Err(error.unwrap());
        }
        let addr: SocketAddr = format!("{host}:{port}").as_str().parse().unwrap();
        let listener = TcpListener::bind(&addr);
        match listener.await {
            Ok(l) => break l,
            Err(e) => error = Some(e.into())
        }

        if begin_range {
            port += 1;
        } else {
            warn!("Cannot start server at {}:{} Retrying at a random port.", &host, port);
            begin_range = true;
            port = min_port;
        }
    };

    let timeout_fmt = if env.config.cyclone.mandatory_timeout_ms == 0 {
        "0 (NO TIMEOUT)".to_string()
    } else {
        let ms = env.config.cyclone.mandatory_timeout_ms;
        format!("{} ms ({} secs)", ms, Duration::from_millis(ms as u64).as_secs())
    };

    println!(
        "\nStarting Cyclone execution server at: http://{host}:{port}\nCyclone instance: {:?}\nWorking Directory: {:?}\nExecution Timeout: {}\nDisabled Options: {}\nVersion: {:?}\n\n{}{}{}",
        &env.cyclone_executable,
        &env.source_path,
        timeout_fmt,
        &env.config.cyclone.disabled_options.join(", "),
        &env.summary.version,
        if env.config.cyclone.disable_syntax_check { "! Syntax check is OFF\n" } else { "" },
        if env.config.cyclone.delete_after_exec { "" } else { "! Automatic file clearing is OFF\n" },
        if env.config.cyclone.censor_system_paths { "" } else { "! Censoring system file paths is OFF: make sure server is only accessible by TRUSTED devices\n" }
    );

    // since the url format is simple, there is no need to use a library to encode it
    let url = format!("{}?set_exec_server=http%3A%2F%2F{}%3A{}", EDITOR_URL, if host == "0.0.0.0" { "127.0.0.1" } else {&host}, port);
    println!("* Connect to online editor: open a browser and visit:\n*\t{}\n", &url);

    let state = Arc::new(env);
    info!("Begin serving {}:{}", &host, port);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let env = state.clone();
        tokio::task::spawn(async move {
            let service = service_fn(|req| {
                handle_api(req, env.clone())
            });
            if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                error!("Server Error: {:?}", e);
            }
        });
    }
}
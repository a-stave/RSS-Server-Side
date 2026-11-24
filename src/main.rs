#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::{TokioIo, TokioTimer};
use reqwest::Client;
use tokio::net::TcpListener;
use tokio::sync::Notify;

struct AppState {
    notify: Notify,
}

const LONG_POLL_TIMEOUT: Duration = Duration::from_secs(30);

//Async function for long polling
//Really basic version of long polling which notifies client when webpage changed
//TODO for this function:
//
async fn long_poll(
    state: Arc<AppState>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let message = tokio::select! {
        _ = state.notify.notified() => "changed",

        _ = tokio::time::sleep(LONG_POLL_TIMEOUT) => "timeout",
    };

    let mut response = Response::new(Full::new(Bytes::from(message)));

    response.headers_mut().insert(
        hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        hyper::header::HeaderValue::from_static("*"),
    );

    Ok(response)
}


//Async function which fetchs url from the argument and return the response
//TODO for this function:
//Modify the function to process query without forcing the url param to be first
async fn fetch_url(
    req: Request<impl hyper::body::Body>,
    client: &Client,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let query = req.uri().query().unwrap_or("");

    let url = query.strip_prefix("url=").unwrap_or("");

    if url.is_empty() {
        return Ok(Response::new(Full::new(Bytes::from(
            "Missing url parameter",
        ))));
    }

    match client.get(url).send().await {
        Ok(resp) => {
            let text = resp.text().await.unwrap_or_default();

            Ok(Response::new(Full::new(Bytes::from(text))))
        }
        Err(e) => Ok(Response::new(Full::new(Bytes::from(format!(
            "Error fetching url: {}",
            e
        ))))),
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // This address is localhost
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // Create a reqwest client to be used for fetching URLs
    let client = Client::new();

    // Shared state for all connections. 
    // Wrapped in an Arc so each spawned task can hold a clone with less resource
    let state = Arc::new(AppState {
        notify: Notify::new(),
    });


    // Simple loop to test long polling
    // Run index.html to verify if client receives data from the server
    let monitor_state = state.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
            println!("Website changed");
            monitor_state.notify.notify_waiters();
        }
    });


    // Bind to the port and listen for incoming TCP connections
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        // When an incoming TCP connection is received grab a TCP stream for
        // client<->server communication.
        let (tcp, _) = listener.accept().await?;
        
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(tcp);

        // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
        // current task without waiting for the processing of the HTTP1 connection we just received
        // to finish
        tokio::task::spawn({
            let client = client.clone();
            let state = state.clone();
            async move {
                // Handle the connection from the client using HTTP1 and pass any
                // HTTP requests received on that connection to the `hello` function
                if let Err(err) = http1::Builder::new()
                    .timer(TokioTimer::new())
                    .serve_connection(
                        io, 
                        service_fn(move |req| {
                            //These are cloned again so they own its references
                            let client = client.clone();
                            let state = state.clone();
                            
                            async move {
                                match req.uri().path() {
                                    "/fetch" => fetch_url(req, &client).await,
                                    "/updates" => long_poll(state).await,
                                    _ => {
                                        let mut response = Response::new(Full::new(Bytes::from("Not Found")));
                                        *response.status_mut() = hyper::StatusCode::NOT_FOUND;
                                        Ok(response)
                                    }
                                }
                            }
                        }),
                    )
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            }
        });
    }
}

/* ------------------ Tests ------------------ */
#[cfg(test)]
mod tests {

    // Unit tests are pretty simple and consist of three things:
    //  1. A descriptive name so that if it fails you can tell what went wrong at CL
    //  2. A function call, with some inputs (hardcoded, generated, or test fixture)
    //  3. An assertion that the output is what you expect
    #[test]
    #[ignore = "Example only: Use this to create unit tests in future modules"]
    fn descriptive_test_name() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

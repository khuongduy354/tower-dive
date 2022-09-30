use futures::future::BoxFuture;
use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;
use hyper::{Body, Error, Request, Response, Server};
use log::info;
use std::convert::Infallible;
use std::future::{self, ready, Future, Ready};
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

// async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new(Body::from("Hello World")))
// }

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(HelloWorld) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

struct HelloWorld;

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        Box::pin(async move {
            info!("receiving request ");
            let resp = ready(Ok(Response::builder()
                .status(200)
                .body(Body::from("Hello World"))
                .expect("Unable to create `http::Response`")))
            .await;

            info!("finished building response:");
            resp
        })
    }
}

// struct LogService<S> {
//     service: S,
// }
//
// impl<S, B> Service<Request<B>> for LogService<S>
// where
//     S: Service<Request<B>>,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }
//
//     fn call(&mut self, request: Request<B>) -> Self::Future {
//         info!("received request");
//         self.service.call(request)
//     }
// }

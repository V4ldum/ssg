use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use ssg::markdown::MarkdownPlugin;
use ssg::pickers::file_picker::FileFilter;
use ssg::pickers::FilePicker;
use ssg::pretty_url::PrettyUrlPlugin;
use ssg::tailwind::TailwindPlugin;
use ssg::template::TemplatePlugin;
use ssg::{err, info, SSGPipelineBuilder};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let ssg = SSGPipelineBuilder::new()
        .plugin(MarkdownPlugin::new())
        .plugin(TemplatePlugin::new())
        .plugin(TailwindPlugin::new())
        .plugin(PrettyUrlPlugin::new())
        .picker(FilePicker::new(FileFilter::Directory("public")))
        .build();

    ssg.run();

    // TODO serve CLI
    info!("Serving HTTP server");
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await.unwrap(); // todo unwrap
    info!("Listening on http://{}", addr);

    loop {
        let (tcp, _) = listener.accept().await.unwrap(); // todo unwrap
        let io = TokioIo::new(tcp);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(serve_file))
                .await
            {
                err!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn serve_file(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path();

    let result =
        fs::read(format!("build/{path}")).or_else(|_| fs::read(format!("build/{path}/index.html")));

    match result {
        Ok(content) => Ok(Response::new(Full::new(Bytes::from(content)))),
        Err(_) => Ok(Response::new(Full::new(Bytes::from("Not Found")))),
    }
}

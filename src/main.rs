extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;

use hyper::{Url, Method, Error};
use hyper::client::{Client, Request};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::mime::Mime;
use hyper_tls::HttpsConnector;

fn main() {
    let url = Url::parse("https://api.github.com/user").unwrap();
    let mut req = Request::new(Method::Get, url);
    let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
    /let token = String::from("token {Your_Token_Here}");
    req.headers_mut().set(UserAgent(String::from("github-rs")));
    req.headers_mut().set(Accept(vec![qitem(mime)]));
    req.headers_mut().set(Authorization(token));

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4,&handle))
        .build(&handle);
    let work = client.request(req)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            // Yo dawg I heard you like futures, so I put a
            // future in your future so that you can asynchronously
            // process code
            res.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            }).and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, Error>(s)
            })
        });
    let user = event_loop.run(work).unwrap();
    println!("We've made it outside the request! \
              We got back the following from our \
              request:\n");
    println!("{}", user);
}

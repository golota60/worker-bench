use std::{collections::HashMap, error::Error};

use lol_html::{element, HtmlRewriter, Settings};
use reqwest::Client;
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    println!("received request");
    if !matches!(req.method(), Method::Get) {
        return Response::error("Method Not Allowed", 405);
    }
    let mut output = vec![];

    let url = req.url().unwrap();
    println!("test url {:?}", url);

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("a[href]", |el| {
                let href = el
                    .get_attribute("href")
                    .expect("href was required")
                    .replace("http:", "https:");

                el.set_attribute("href", &href)?;

                Ok(())
            })],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );

    let client = Client::new();
    let resp = client.get("https://szymon.codes").send().await.unwrap();

    let bytes = resp.bytes().await.unwrap();

    Response::from_html(String::from_utf8(bytes.as_ref().into()).unwrap())

    // format!("https://szymon.codes/{:?}", url)
    // let resp = reqwest::get("https://example.com").await.expect("AAA");
    // let parsed = resp.json::<HashMap<String, String>>().await.expect("BBB");

    // let resp = reqwest::blocking::get(format!("https://szymon.codes/{:?}", url))
    //     .unwrap()
    //     .json::<HashMap<String, String>>()
    //     .expect("Making request to upstream failed");

    // rewriter.write(b"<div><a href=").unwrap();
    // rewriter.write(b"http://example.com>").unwrap();
    // rewriter.write(b"</a></div>").unwrap();
    // rewriter.end().unwrap();

    // assert_eq!(
    //     String::from_utf8(output).unwrap(),
    //     r#"<div><a href="https://example.com"></a></div>"#
    // );

    // Ok(())
    // Response::ok("Hello, Worrld!")
}

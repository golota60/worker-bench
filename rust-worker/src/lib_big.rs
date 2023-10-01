use std::collections::HashMap;

use async_trait::async_trait;
use lol_html::{element, rewrite_str, RewriteStrSettings};
use reqwest::Client;
use worker::*;

const PROXY_LINK: &str = "szymon.codes";
const CMS_ATTRIBUTE: &str = "turbo-cms-tag";

fn get_mocked_data() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("page_title", "page_title"),
        ("main_header", "main_header"),
        ("main_description", "main_description"),
        ("primary_cta_link", "primary_cta_link"),
        ("primary_cta_text", "primary_cta_text"),
        ("secondary_cta_link", "secondary_cta_link"),
        ("secondary_cta_text", "secondary_cta_text"),
        ("info_1_text", "info_1_text"),
        ("info_1_desc", "info_1_desc"),
        ("info_2_text", "info_2_text"),
        ("info_2_desc", "info_2_desc"),
        ("info_3_text", "info_3_text"),
        ("info_3_desc", "info_3_desc"),
        ("review_title", "review_title"),
        ("review_1_name", "review_1_name"),
        ("review_1_position", "review_1_position"),
        ("review_1_desc", "review_1_desc"),
        ("review_2_name", "review_2_name"),
        ("review_2_position", "review_2_position"),
        ("review_2_desc", "review_2_desc"),
        ("review_3_name", "review_3_name"),
        ("review_3_position", "review_3_position"),
        ("review_3_desc", "review_3_desc"),
        ("review_4_name", "review_4_name"),
        ("review_4_position", "review_4_position"),
        ("review_4_desc", "review_4_desc"),
        ("review_5_name", "review_5_name"),
        ("review_5_position", "review_5_position"),
        ("review_5_desc", "review_5_desc"),
        ("review_6_name", "review_6_name"),
        ("review_6_position", "review_6_position"),
        ("review_6_desc", "review_6_desc"),
        ("main_header_2", "main_header_2"),
        ("main_header_3", "main_header_3"),
        ("main_header_4", "main_header_4"),
        ("main_description_2", "main_description_2"),
        ("main_description_3", "main_description_3"),
        ("main_description_4", "main_description_4"),
        ("primary_cta_link_2", "primary_cta_link_2"),
        ("primary_cta_link_3", "primary_cta_link_3"),
        ("primary_cta_link_4", "primary_cta_link_4"),
        ("primary_cta_text_2", "primary_cta_text_2"),
        ("primary_cta_text_3", "primary_cta_text_3"),
        ("primary_cta_text_4", "primary_cta_text_4"),
        ("secondary_cta_link_2", "secondary_cta_link_2"),
        ("secondary_cta_link_3", "secondary_cta_link_3"),
        ("secondary_cta_link_4", "secondary_cta_link_4"),
        ("secondary_cta_text_2", "secondary_cta_text_2"),
        ("secondary_cta_text_3", "secondary_cta_text_3"),
        ("secondary_cta_text_4", "secondary_cta_text_4"),
        ("info_1_text_2", "info_1_text_2"),
        ("info_1_text_3", "info_1_text_3"),
        ("info_1_text_4", "info_1_text_4"),
        ("info_1_desc_2", "info_1_desc_2"),
        ("info_1_desc_3", "info_1_desc_3"),
        ("info_1_desc_4", "info_1_desc_4"),
        ("info_2_text_2", "info_2_text_2"),
        ("info_2_text_3", "info_2_text_3"),
        ("info_2_text_4", "info_2_text_4"),
        ("info_2_desc_2", "info_2_desc_2"),
        ("info_2_desc_3", "info_2_desc_3"),
        ("info_2_desc_4", "info_2_desc_4"),
        ("info_3_text_2", "info_3_text_2"),
        ("info_3_text_3", "info_3_text_3"),
        ("info_3_text_4", "info_3_text_4"),
        ("info_3_desc_2", "info_3_desc_2"),
        ("info_3_desc_3", "info_3_desc_3"),
        ("info_3_desc_4", "info_3_desc_4"),
        ("review_title_2", "review_title_2"),
        ("review_title_3", "review_title_3"),
        ("review_title_4", "review_title_4"),
        ("review_1_name_2", "review_1_name_2"),
        ("review_1_name_3", "review_1_name_3"),
        ("review_1_name_4", "review_1_name_4"),
        ("review_1_position_2", "review_1_position_2"),
        ("review_1_position_3", "review_1_position_3"),
        ("review_1_position_4", "review_1_position_4"),
        ("review_1_desc_2", "review_1_desc_2"),
        ("review_1_desc_3", "review_1_desc_3"),
        ("review_1_desc_4", "review_1_desc_4"),
        ("review_2_name_2", "review_2_name_2"),
        ("review_2_name_3", "review_2_name_3"),
        ("review_2_name_4", "review_2_name_4"),
        ("review_2_position_2", "review_2_position_2"),
        ("review_2_position_3", "review_2_position_3"),
        ("review_2_position_4", "review_2_position_4"),
        ("review_2_desc_2", "review_2_desc_2"),
        ("review_2_desc_3", "review_2_desc_3"),
        ("review_2_desc_4", "review_2_desc_4"),
        ("review_3_name_2", "review_3_name_2"),
        ("review_3_name_3", "review_3_name_3"),
        ("review_3_name_4", "review_3_name_4"),
        ("review_3_position_2", "review_3_position_2"),
        ("review_3_position_3", "review_3_position_3"),
        ("review_3_position_4", "review_3_position_4"),
        ("review_3_desc_2", "review_3_desc_2"),
        ("review_3_desc_3", "review_3_desc_3"),
        ("review_3_desc_4", "review_3_desc_4"),
        ("review_4_name_2", "review_4_name_2"),
        ("review_4_name_3", "review_4_name_3"),
        ("review_4_name_4", "review_4_name_4"),
        ("review_4_position_2", "review_4_position_2"),
        ("review_4_position_3", "review_4_position_3"),
        ("review_4_position_4", "review_4_position_4"),
        ("review_4_desc_2", "review_4_desc_2"),
        ("review_4_desc_3", "review_4_desc_3"),
        ("review_4_desc_4", "review_4_desc_4"),
        ("review_5_name_2", "review_5_name_2"),
        ("review_5_name_3", "review_5_name_3"),
        ("review_5_name_4", "review_5_name_4"),
        ("review_5_position_2", "review_5_position_2"),
        ("review_5_position_3", "review_5_position_3"),
        ("review_5_position_4", "review_5_position_4"),
        ("review_5_desc_2", "review_5_desc_2"),
        ("review_5_desc_3", "review_5_desc_3"),
        ("review_5_desc_4", "review_5_desc_4"),
        ("review_6_name_2", "review_6_name_2"),
        ("review_6_name_3", "review_6_name_3"),
        ("review_6_name_4", "review_6_name_4"),
        ("review_6_position_2", "review_6_position_2"),
        ("review_6_position_3", "review_6_position_3"),
        ("review_6_position_4", "review_6_position_4"),
        ("review_6_desc_2", "review_6_desc_2"),
        ("review_6_desc_3", "review_6_desc_3"),
        ("review_6_desc_4", "review_6_desc_4"),
    ])
}

#[async_trait(?Send)]
trait Workerable {
    /// Consumes the value and converts it to worker response
    async fn to_worker_response(self) -> worker::Response;
}

#[async_trait(?Send)]
impl Workerable for reqwest::Response {
    async fn to_worker_response(self) -> worker::Response {
        let headers = self.headers().clone();
        let status = self.status().clone();
        let byt = self.bytes();
        let as_bytes = byt.await;
        let as_bytes = as_bytes.unwrap();
        let as_bytes: Vec<u8> = as_bytes.into();

        let rep = worker::Response::from_bytes(as_bytes).unwrap();
        let rep = rep.with_headers(headers.into());
        let rep = rep.with_status(status.as_u16());
        rep
    }
}

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if !matches!(req.method(), Method::Get) {
        return Response::error("Method Not Allowed", 405);
    }
    let mocked_data = get_mocked_data();

    let mut url = req.url().unwrap();
    _ = url.set_host(Some(PROXY_LINK));

    let client = Client::new();
    let resp = client.get(url).send().await.unwrap();

    let headers = resp.headers().clone();
    let status = resp.status().clone();

    // If we're not processing an html file, bail early
    let content_type = headers.get("Content-Type");
    if let Some(content_type) = content_type {
        if !content_type.to_str().unwrap().contains("text/html") {
            return Ok(resp.to_worker_response().await);
        }
    }

    let as_text = resp.text();
    let as_html_string = as_text.await;
    let as_html_string = as_html_string.unwrap();
    let output = rewrite_str(
        as_html_string.as_str(),
        RewriteStrSettings {
            element_content_handlers: vec![element!(format!("[{}]", CMS_ATTRIBUTE), |el| {
                let cms_tag_attr = el.get_attribute(CMS_ATTRIBUTE).unwrap();
                let target_val = mocked_data.get(cms_tag_attr.as_str());

                if let Some(target_val) = target_val {
                    if cms_tag_attr.contains("link") {
                        _ = el.set_attribute("href", target_val);
                    } else {
                        el.set_inner_content(target_val, lol_html::html_content::ContentType::Text);
                    }
                }

                Ok(())
            })],
            ..RewriteStrSettings::default()
        },
    )
    .expect("error while rewriting");

    let rep = worker::Response::from_html(output).unwrap();
    let rep = rep.with_headers(headers.into());
    let rep = rep.with_status(status.as_u16());

    Ok(rep)
}

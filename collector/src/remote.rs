use crate::collect_business_obj;
use crate::error::Error;
use crate::raw::PrizePage;
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::redirect::Policy;
use ssq_tool_domain::PrBusinessObj;
use tracing::trace;

const REMOTE_URL: &str = "https://www.cwl.gov.cn/cwl_admin/front/cwlkj/search/kjxx/findDrawNotice";

pub(super) async fn collect_from_network(
    record_size_to_store: Option<usize>,
) -> Result<Vec<PrBusinessObj>, Error> {
    let mut default_headers_map = HeaderMap::new();
    default_headers_map.insert(header::ACCEPT, HeaderValue::from_static("*/*"));
    default_headers_map.insert(
        header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br, x-gzip"),
    );
    let collect_remote_client = reqwest::Client::builder()
        .default_headers(default_headers_map)
        .no_proxy()
        .cookie_store(true)
        .user_agent("SSQ-TOOL")
        .redirect(Policy::default())
        .build()?;
    let remote_response = collect_remote_client
        .get(REMOTE_URL)
        .query(&[
            ("name", "ssq"),
            ("pageNo", "1"),
            ("pageSize", "99999999"),
            ("systemType", "PC"),
        ])
        .send()
        .await?;

    let response_header = remote_response.headers();
    trace!("远程响应头部：{response_header:?}");
    remote_response.cookies().for_each(|cookie| {
        trace!("远程响应 Cookie：{cookie:?}");
    });
    let response_body = remote_response.text().await?;
    trace!("远程响应数据：\n{response_body}");
    let page = serde_json::from_str::<PrizePage>(&response_body)?;
    collect_business_obj(page, record_size_to_store)
}

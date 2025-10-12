use crate::collect_business_obj;
use crate::error::Error;
use crate::raw::PrizePage;
use reqwest::redirect::Policy;
use reqwest::ClientBuilder;
use ssq_tool_domain::PrBusinessObj;
use tracing::debug;

const REMOTE_URL: &str = "https://www.cwl.gov.cn/cwl_admin/front/cwlkj/search/kjxx/findDrawNotice";

pub(super) async fn collect_from_network(
    record_size_to_store: Option<usize>,
) -> Result<Vec<PrBusinessObj>, Error> {
    let collect_remote_client = ClientBuilder::new()
        .no_proxy()
        .cookie_store(true)
        .user_agent("SSQ-TOOL")
        .redirect(Policy::none())
        .build()?;
    let remote_response = collect_remote_client.get(REMOTE_URL)
        .query(&[
            ("name", "ssq"),
            ("pageNo", "1"),
            ("pageSize", "99999999"),
            ("systemType", "PC"),
        ])
        .header("Accept", "*/*")
        .header("Accept-Encoding", "gzip, deflate, br, x-gzip")
        .header("Cookie", "HMF_CI=a2bd4469ecff2e28320770e0feb73e987af53fe197afa7ab4a98436df85b87365f9f9184e11f52db215aa67be3720088fbadfa25ca2892f121ae99f75464b7200a")
        .send()
        .await?;

    let response_body = remote_response.text().await?;
    debug!("远程数据：\n{response_body}");
    let page = serde_json::from_str::<PrizePage>(&response_body)?;
    collect_business_obj(page, record_size_to_store)
}

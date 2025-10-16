use chrono::NaiveDate;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use rmcp::{ErrorData, ServerHandler, schemars, tool, tool_handler, tool_router};
use ssq_tool_domain::PrBusinessObj;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FilterByWeekRequest {
    #[schemars(description = "星期几")]
    pub day: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FilterBySeqRangeRequest {
    #[schemars(description = "起始序号")]
    pub start: usize,
    #[schemars(description = "结束序号")]
    pub end: usize,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FilterByDateRangeRequest {
    #[schemars(description = "起始日期")]
    pub start: NaiveDate,
    #[schemars(description = "结束日期")]
    pub end: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct SsqMcpService<'a> {
    prize_record_business_obj: &'a [PrBusinessObj],
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl<'a> SsqMcpService<'a>
where
    'a: 'static,
{
    pub fn new(prize_record_business_obj: &'a [PrBusinessObj]) -> Self {
        Self {
            prize_record_business_obj,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "取得所有官方中奖数据")]
    pub async fn get_all_official_data(&self) -> Result<CallToolResult, ErrorData> {
        let call_tool_result =
            CallToolResult::success(vec![Content::json(self.prize_record_business_obj)?]);
        return Ok(call_tool_result);
    }

    #[tool(description = "按照星期几来取得官方中奖数据")]
    pub async fn get_official_data_by_day(
        &self,
        param: Parameters<FilterByWeekRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        let prize_record_business_objs = self
            .prize_record_business_obj
            .iter()
            .filter(|reocrd| reocrd.day.eq(&param.0.day))
            .collect::<Vec<&PrBusinessObj>>();
        let call_tool_result =
            CallToolResult::success(vec![Content::json(prize_record_business_objs)?]);
        return Ok(call_tool_result);
    }

    #[tool(description = "按照序号范围来取得官方中奖数据")]
    pub async fn get_official_data_by_seq_range(
        &self,
        param: Parameters<FilterBySeqRangeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        let prize_record_business_objs = self
            .prize_record_business_obj
            .iter()
            .filter(|reocrd| reocrd.seq >= param.0.start && reocrd.seq <= param.0.end)
            .collect::<Vec<&PrBusinessObj>>();
        let call_tool_result =
            CallToolResult::success(vec![Content::json(prize_record_business_objs)?]);
        return Ok(call_tool_result);
    }

    #[tool(description = "按照日期范围来取得官方中奖数据")]
    pub async fn get_official_data_by_date_range(
        &self,
        param: Parameters<FilterByDateRangeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        let prize_record_business_objs = self
            .prize_record_business_obj
            .iter()
            .filter(|reocrd| reocrd.date >= param.0.start && reocrd.date <= param.0.end)
            .collect::<Vec<&PrBusinessObj>>();
        let call_tool_result =
            CallToolResult::success(vec![Content::json(prize_record_business_objs)?]);
        return Ok(call_tool_result);
    }
}

#[tool_handler]
impl<'a> ServerHandler for SsqMcpService<'a>
where
    'a: 'static,
{
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("双色球数据分析服务".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

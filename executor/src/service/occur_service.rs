use actix_web::web::Json;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use rmcp::model::{ServerCapabilities, ServerInfo};

#[derive(Debug, Clone)]
pub struct OccurMcpService{
    tool_router: ToolRouter<Self>,
}



#[tool_router]
impl OccurMcpService {
    pub fn new()->Self{
        Self{
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "取得蓝球的出现情况")]
    pub async fn get_blue_ball_occurs(&self) -> Json<String> {
        Json("abc".to_owned())
    }
}

#[tool_handler]
impl ServerHandler for OccurMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("双色球出现情况服务".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}



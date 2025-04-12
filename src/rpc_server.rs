
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use anyhow::{anyhow, Result};
use axum::{routing::post, Json, Router};
use tower::ServiceBuilder;

use crate::{
    config::Config,
    ledger::Ledger,
    rpc_handler::RpcRequestHandler,
    rpc_router::RpcRouter,
    rpc_request::JsonRpcRequest,
    rpc_response::{JsonRpcResponse},
};

/// Holds references to everything the service needs at runtime.
#[derive(Clone)]
pub struct ApplicationState {
    pub ledger: Arc<Ledger>,
}

/// Main struct that runs the entire server.
pub struct RpcServer {
    config: Config,
    state: Option<ApplicationState>,
    router: Option<Arc<RpcRouter>>,
}

impl RpcServer {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: None,
            router: None,
        }
    }

    /// Runs the server
    pub async fn run(&mut self) -> Result<()> {
        // Open ledger in secondary mode
        let ledger = Ledger::open_ledger(&self.config)?;
        let state = ApplicationState { ledger };
        self.state = Some(state.clone());

        // Create RPC request handler
        let handler = Arc::new(RpcRequestHandler::new());

        // Create the router
        let router = Arc::new(RpcRouter::new(handler));
        self.router = Some(router);

        // Start the Axum server
        self.start_http_server().await
    }

    async fn start_http_server(&self) -> Result<()> {
        let st = self
            .state
            .as_ref()
            .ok_or_else(|| anyhow!("State not initialized"))?;
        let router = self
            .router
            .as_ref()
            .ok_or_else(|| anyhow!("Router not initialized"))?;

        let http_app = Router::new()
            .route("/", post({
                let router = Arc::clone(router);
                move |axum::extract::State(s): axum::extract::State<ApplicationState>,
                      Json(req): Json<JsonRpcRequest>|
                    {
                        let router = Arc::clone(&router);
                        async move {
                            let response: JsonRpcResponse<_> = router.handle(&s, &req).await;
                            Json(response)
                        }
                    }
            }))
            .with_state(st.clone())
            .layer(ServiceBuilder::new());

        let ip: IpAddr = self
            .config
            .bind_addr
            .parse()
            .map_err(|e| anyhow!("Invalid bind address '{}': {}", self.config.bind_addr, e))?;
        let addr = SocketAddr::new(ip, self.config.port);

        log::info!("RPC server listening on http://{}", addr);

        axum::Server::bind(&addr)
            .serve(http_app.into_make_service())
            .await
            .map_err(|e| anyhow!("Server error: {:?}", e))
    }
}
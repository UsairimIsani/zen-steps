use axum::Router;
use crate::config::EnvConfig;
use crate::error::Error;
use crate::routes;
use crate::shared::AppState;

pub async  fn run_server() -> Result<(), Error>{
    let config: EnvConfig = envy::from_env().expect("Loading server config failed");
    let state = AppState::new(config.clone());

    let app = Router::new().nest("/api/v1", routes::public(state));

    let server = axum::Server::bind(&config.sock_addr())
        .serve(app.into_make_service());

    if let Err(e) = server.await {
        tracing::error!("Server Error :{e:?}");
    }

    Ok(())
}
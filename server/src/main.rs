use cli::Opt;
use config::{env::Env, log};
use graphql::generate_schema;
use structopt::StructOpt;

mod app;
mod bootstrap;
mod cli;
mod config;
mod di;
mod graphql;
mod lib;
mod routes;

#[tokio::main]
async fn main() {
    // envファイルの読み込み
    Env::init();

    // ログの設定
    log::init_logging();

    let opt = Opt::from_args();
    match opt {
        Opt::Serve => {
            bootstrap::boot().await;
        }
        Opt::GraphQlSchema => {
            generate_schema().await;
        }
    }
}

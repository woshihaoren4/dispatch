mod app;
mod cmd;
mod conf;
mod infra;
mod pb;

#[tokio::main]
async fn main() {
    cmd::start().await;
}

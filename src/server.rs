mod cmd;
mod pb;
mod app;
mod infra;

#[tokio::main]
async fn main()  {
    cmd::start().await;
}
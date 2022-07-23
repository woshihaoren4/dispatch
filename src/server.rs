mod cmd;
mod pb;
mod app;

#[tokio::main]
async fn main()  {
    cmd::start().await;
}
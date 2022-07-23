mod run;

pub async fn start(){
    let cmd_run= run::AppRun::new();

    wd_run::ArgsManager::new()
        .register_cmd(cmd_run.args(),cmd_run)
        .run().await;
}
mod on_exit;
mod run;

pub async fn start() {
    let (cmd_run, receiver) = run::AppRun::new();
    let exit = on_exit::AppExit { receiver };

    wd_run::ArgsManager::new()
        .register_cmd(cmd_run.args(), cmd_run)
        .register_exit(exit)
        .run()
        .await;
}

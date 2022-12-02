use std::future::Future;
use std::pin::Pin;
use async_channel::Receiver;
use wd_run::Context;

pub struct AppExit{
    pub(crate) receiver:Receiver<Box<dyn wd_run::EventHandle + Sync + Send+ 'static>>
}

impl wd_run::EventHandle for AppExit {
    fn handle(&self, ctx: Context) ->Pin<Box<dyn Future<Output = Context> + Send>> {
        let chan = self.receiver.clone();
        Box::pin(async move{
            while let i = chan.try_recv() {
                if let Ok(o) = i {
                    o.handle(ctx.clone()).await;
                }else{
                    break
                }
            }
            return ctx
        })
    }
}
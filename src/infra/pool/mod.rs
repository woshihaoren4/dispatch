mod pool_entity;
mod pool_interface;

pub use pool_entity::*;
pub use pool_interface::*;

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn pool_test(){
        let p = Pool::new();
        let str = "hello";
        let result = p.push::<String,_>(async move{
            Ok(format!("{} {}",str,"world"))
        }).await.unwrap();
        assert_eq!(result,stringify!(hello world),"result={}",result)
    }
}

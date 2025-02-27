mod hook;
mod keys;
pub use keys::VirtualKey;
pub use hook::KeysWatcher;





#[cfg(test)]
mod tests
{
    use std::time::Duration;
    use crate::{hook::KeysWatcher, keys::VirtualKey};

    #[tokio::test]
    async fn run_test()
    {
        let _ = logger::StructLogger::new_default();
        let mut key_watcher = KeysWatcher::new();
        key_watcher
        .register(&[VirtualKey::LeftCtrl, VirtualKey::LeftAlt], callback_1)
        .register(&[VirtualKey::F5, VirtualKey::MouseLeftClick], callback_2)
        .watch();
        loop 
        {
            tokio::time::sleep(Duration::from_millis(5000)).await;
        }
    }
    async fn callback_1()
    {
        logger::info!("left control + left alt!");
    }
    async fn callback_2()
    {
        logger::info!("F5 + mouse left click");
    }
}
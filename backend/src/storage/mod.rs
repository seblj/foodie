use hyper::Method;
use uuid::Uuid;

pub mod aws;

#[async_trait::async_trait]
pub trait FoodieStorage {
    async fn get_presigned_url(&self, file: Uuid, method: Method) -> Result<String, anyhow::Error>;

    async fn delete(&self, file: Uuid) -> Result<(), anyhow::Error>;

    // async fn save_file<T, U>(&self, file: &str, body: T) -> Result<(), anyhow::Error>
    // where
    //     T: Stream<Item = Result<Bytes, U>> + Send + Unpin,
    //     U: Send;

    // async fn delete_file(&self, file: &str) -> Result<(), anyhow::Error>;
}

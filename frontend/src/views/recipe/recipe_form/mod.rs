use std::time::Duration;

use common::recipe::RecipeImage;
use uuid::Uuid;
use web_sys::File;

use crate::{
    context::toast::{use_toast, Toast, ToastType, ToasterTrait},
    request::get,
};

pub mod recipe_info;
pub mod recipe_ingredients;
pub mod recipe_steps;

pub async fn try_upload_image(file: Option<File>) -> Result<Option<Uuid>, anyhow::Error> {
    let toast = use_toast().unwrap();

    let Some(file) = file else {
        return Ok(None);
    };

    let image = match get("/api/recipe/image").send().await {
        Ok(res) if res.ok() => res.json::<RecipeImage>().await?,
        _ => {
            toast.add(Toast {
                ty: ToastType::Error,
                body: "Failed to upload image".to_string(),
                timeout: Some(Duration::from_secs(5)),
            });
            return Err(anyhow::anyhow!("Couldn't upload file"));
        }
    };

    reqwasm::http::Request::put(&image.url)
        .body(file.value_of())
        .send()
        .await?;

    Ok(Some(image.id))
}

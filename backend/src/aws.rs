// #[cfg(test)]
// mod tests {
//     use aws_sdk_s3::Client as S3Client;
//     #[tokio::test(flavor = "multi_thread")]
//     async fn test_upload_image() -> Result<(), anyhow::Error> {
//         let client = S3Client::from_conf(
//             aws_sdk_s3::config::Builder::new()
//                 .endpoint_url("http://localhost:4566/".to_string())
//                 .force_path_style(true)
//                 .build(),
//         );

//         let resp = client
//             .delete_object()
//             .bucket("images")
//             .key("foo")
//             .send()
//             .await?;
//         println!("resp: {:?}", resp);

//         // let body = ByteStream::from_path(Path::new(
//         //     "/Users/sebastianlyngjohansen/projects/foodie/sea.jpg",
//         // ))
//         // .await?;

//         // let resp = client
//         //     .put_object()
//         //     .bucket("images")
//         //     .key("foo")
//         //     .body(body)
//         //     .send()
//         //     .await
//         //     .unwrap();

//         // println!("resp: {:?}", resp);

//         // client
//         //     .put_object()
//         //     .bucket(bucket_name)
//         //     .key("foo")
//         //     .body(body)
//         //     .send()
//         //     .await?;

//         Ok(())
//     }
// }

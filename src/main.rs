// mod dataframe;

// use dataframe::{
//     DataFrameOperations,
//     LazyDataFrameOps
//     };
// use polars::prelude::*;

// fn main() -> Result<(), PolarsError> {
    // Eager CSV loading
    // let df = LazyDataFrameOps::load_csv("very_large_dataset.csv")?;
    // let df = df.filter(col("value1").gt(30)) // Example eager operation
    // println!("Eager DataFrame:\n{:?}", df.head(Some(5)));

    // Lazy CSV loading
    // let lazy_df = LazyDataFrameOps::load_csv_lazy("very_large_dataset.csv")?
    //     .filter(col("value1").gt(30)) // Example lazy operation
    //     .collect()?; // Execute lazy operations and materialize DataFrame
    // let lazy_df = LazyDataFrameOps::load_csv_lazy("very_large_dataset.csv")?
    //     .filter(
    //         col("value1").gt(30)
    //     )
    //     .group_by(["category"])
    //     .agg([
    //         col("value2").sum().alias("sum_value2"),
    //     ])
    //     .collect()?;
    // println!("Lazy DataFrame:\n{:?}", lazy_df);

//     Ok(())
// }

mod dataframe;


use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;
use dataframe::{DataFrameOperations, LazyDataFrameOps};
use polars::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), PolarsError> {

    let url = "http://localhost:9000".to_string();
    let cred = Credentials::new("admin", "admin123", None, None, "minio");
    let s3_config = aws_sdk_s3::config::Builder::new()
            .endpoint_url(url)
            .credentials_provider(cred)
            .region(Region::new("eu-central-1"))
            .force_path_style(true)
            .build();
    let client = aws_sdk_s3::Client::from_conf(s3_config);


    let bucket = "my-bucket";
    let key = "very_large_dataset.parquet";


    let df = LazyDataFrameOps::load_s3_to_dataframe(&client, bucket, key).await?;


    let filtered_df = df
            .lazy()
            .group_by(["category"])
            .agg([col("value2").sum().alias("sum_value2")])
            .collect()?;
    println!("Filtered DataFrame:\n{:?}", filtered_df);

    Ok(())
}

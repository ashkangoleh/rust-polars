// use polars::prelude::*;

// pub trait DataFrameOperations {
//     fn load_csv_lazy(path: &str) -> Result<LazyFrame, PolarsError>;
//     fn load_csv(path: &str) -> Result<DataFrame, PolarsError>;
// }

// pub struct LazyDataFrameOps; // Marker struct for implementing the trait

// impl DataFrameOperations for LazyDataFrameOps {
//     fn load_csv(path: &str) -> Result<DataFrame, PolarsError> {
//         CsvReadOptions::default()
//             .with_has_header(true)
//             .try_into_reader_with_file_path(Some(path.into()))? // Use the new API
//             .finish()
//     }

//     fn load_csv_lazy(path: &str) -> Result<LazyFrame, PolarsError> {
//         Ok(LazyCsvReader::new(path.to_string())
//             .with_has_header(true) // Correct method for LazyCsvReader
//             .finish()?)
//     }
// }

use aws_sdk_s3::Client;
use polars::prelude::*;
use std::io::Cursor;
// use duckdb::Connection;

pub trait DataFrameOperations {
    fn load_csv_lazy(path: &str) -> Result<LazyFrame, PolarsError>;
    fn load_csv(path: &str) -> Result<DataFrame, PolarsError>;
    async fn load_s3_to_dataframe(client: &Client, bucket: &str, key: &str) -> Result<DataFrame, PolarsError>;
    // fn duckdb_loader(sql: &str);
}

pub struct LazyDataFrameOps;

impl DataFrameOperations for LazyDataFrameOps {
    fn load_csv(path: &str) -> Result<DataFrame, PolarsError> {
        CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()
    }

    fn load_csv_lazy(path: &str) -> Result<LazyFrame, PolarsError> {
        Ok(LazyCsvReader::new(path.to_string())
            .with_has_header(true)
            .finish()?)
    }

    async fn load_s3_to_dataframe(
        client: &Client,
        bucket: &str,
        key: &str,
    ) -> Result<DataFrame, PolarsError> {
        // Fetch the object from S3
        let response = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .expect("Failed to fetch S3 object");

        // Read the file contents into a buffer
        let data = response
            .body
            .collect()
            .await
            .expect("Failed to read object body")
            .into_bytes();
        let cursor = Cursor::new(data);

        // Determine file type by extension
        if key.ends_with(".parquet") {
            ParquetReader::new(cursor).finish()
        } else {
            Err(PolarsError::ComputeError("Unsupported file extension".into()))
        }
    }
    
    // fn duckdb_loader(sql: &str) {
    //     let conn = Connection::open_in_memory().unwrap();
        
    //     let stmt = conn.prepare(sql).unwrap();
    //     // let mut rows = stmt.query([]).unwrap();

    //     // let mut df = DataFrame::default();
    //     println!("{:?}", stmt);
    // }
}

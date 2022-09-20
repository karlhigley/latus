extern crate polars;
use polars::datatypes::{DataType, RevMapping};
use polars::frame::DataFrame;
use polars::prelude::{Arc, ParquetReader, ParquetWriter, SerReader};

use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn read_parquet(file: File) -> DataFrame {
    let buf_reader = BufReader::new(file);

    let reader = ParquetReader::new(buf_reader);
    reader.finish().expect("couldn't read dataframe")
}

pub fn write_parquet(file: File, df: &mut DataFrame) {
    let buf_writer = BufWriter::new(file);
    let writer = ParquetWriter::new(buf_writer);
    writer.finish(df).expect("couldn't write dataframe");
}

pub fn preprocess_df(mut df: DataFrame, vector_col: &str) -> DataFrame {
    // get column names excluding vector column
    let col_names: Vec<String> = df.get_column_names_owned();

    let filtered_col_names: Vec<&String> = col_names
        .iter()
        .map(|name| {
            if *name != *vector_col {
                Some(name)
            } else {
                None
            }
        })
        .flatten()
        .collect();

    // convert columns to categorical
    for column in &filtered_col_names[..] {
        let cat_dtype = &DataType::Categorical(Some(Arc::new(RevMapping::default())));
        df.try_apply(column.as_str(), |s| s.cast(&cat_dtype))
            .expect("couldn't convert column to categorical");
    }

    // sort by categorical columns
    let directions: Vec<bool> = filtered_col_names.iter().map(|_| true).collect();
    df.sort(filtered_col_names, directions)
        .expect("sort failed");
    df
}

#[cfg(test)]
mod tests {
    use super::{preprocess_df, read_parquet, write_parquet};
    use polars::prelude::*;

    #[test]
    fn convert_to_categorical() {
        let s0 = Series::new("fruits", &["banana", "banana", "apple", "apple", "banana"]);
        let s1 = Series::new("cars", &["beetle", "audi", "beetle", "beetle", "beetle"]);
        let vectors = [
            [1., 2., 3.],
            [4., 5., 6.],
            [7., 8., 9.],
            [1., 4., 7.],
            [2., 5., 8.],
        ]
        .map(|l| Series::new("", l));
        let vector_series = Series::new("vectors", vectors);

        let df = DataFrame::new(vec![s0, s1, vector_series]).unwrap();

        let processed_df = preprocess_df(df, &"vectors");

        let fields = processed_df.fields();
        let col_names: Vec<&String> = fields.iter().map(|f| f.name()).collect();

        // Haven't figured out how to assert on the dtype, so this will have to do for now
        assert_eq!(col_names, &["fruits", "cars", "vectors"]);
        assert_eq!(
            processed_df.get_column_names(),
            &["fruits", "cars", "vectors"]
        );

        // let cat_dtype = &DataType::Categorical(Some(Arc::new(RevMapping::default())));
        // assert_eq!(df.ge"fruits")
    }

    #[test]
    fn parquet_buffered_write_and_read() {
        use tempfile::NamedTempFile;

        // Construct a dataframe
        let s0 = Series::new("fruits", &["banana", "banana", "apple", "apple", "banana"]);
        let s1 = Series::new("cars", &["beetle", "audi", "beetle", "beetle", "beetle"]);

        let vectors = [
            [1., 2., 3.],
            [4., 5., 6.],
            [7., 8., 9.],
            [1., 4., 7.],
            [2., 5., 8.],
        ]
        .map(|l| Series::new("", l));
        let vector_series = Series::new("vectors", vectors);

        let mut df = DataFrame::new(vec![s0, s1, vector_series]).unwrap();

        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_read = tmpfile.reopen().unwrap();

        write_parquet(tmpfile.into_file(), &mut df);

        let read_df = read_parquet(tmpfile_read);

        assert_eq!(df, read_df)
    }
}

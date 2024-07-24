use polars::prelude::*;
use rust_ds::{any_to_string, one_hot_encoding};

fn main() {
    // Create a sample DataFrame
    let df = DataFrame::new(vec![
        Series::new("location", &["A", "B", "C", "A", "B"]),
        Series::new("value", &[1, 2, 3, 4, 5]),
    ])
    .unwrap();

    let mut df_encode = df.clone();

    println!("{:?}", df);

    let row_len = df.height();
    println!("row length: {:?}", row_len);
    
    let cols = df.column("location").unwrap().unique().unwrap();
    println!("{:?}", cols);

    let cols_vec: Vec<&str> = cols
        .iter()
        .map(|value| match value {
            AnyValue::String(s) => s,
            _ => panic!("Expected string value"),
        })
        .collect();

    println!("the vector is:\n{:?}", cols_vec);

    for j in 0..cols_vec.len() {
        let mut the_vec = vec![0;df_encode.height()];
        for i in 0..df.height() {
            if any_to_string(df_encode.column("location").unwrap().get(i).unwrap()) == cols_vec[j].to_string() {
                the_vec[i] = 1;
            }
        }
        let the_new_series = Series::new(cols_vec[j], &the_vec);
        let _ = df_encode.with_column(the_new_series);
    }

    println!("{:?}", &df_encode);

    println!("test function:");
    let df_encode2 = one_hot_encoding(df, "location");
    println!("{:?}", &df_encode2); // it outputs the one-hot encoded columns in dataframe format for the selected column

}


use polars::prelude::*;
use rust_ds::train_test_split;

fn main() {
    // Create one sample dataframe
    let df = DataFrame::new(vec![
        Series::new("col_1", &[1,3,5,7,9,11,13,15,17,19]),
        Series::new("col_2", &[2,4,6,8,10,12,14,16,18,20]),
    ])
    .unwrap();

    println!("{:?}", df);

    // Get the number of rows in the DataFrame
    let num_rows = df.height();

    // Shuffle the rows by sampling all rows without replacement
    let shuffled_df = df.sample_n_literal(num_rows,false,true,Some(1)).unwrap();

    println!("{:?}", shuffled_df);
    
    let test_size = 0.333;

    let res = (num_rows as f64 * test_size).round() as usize;

    let df_train = shuffled_df.slice(0i64, num_rows - res);

    println!("{:?}", df_train);

    let df_test = shuffled_df.slice((num_rows - res) as i64, res);

    println!("{:?}", df_test);

    let (df_train2, df_test2) = train_test_split(df, 0.333, true, 1);

    println!("test the function:");
    println!("{:?}", df_train2);
    println!("{:?}", df_test2);

}
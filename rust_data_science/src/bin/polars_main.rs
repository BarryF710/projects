use polars::prelude::*;//{CsvReader, DataType, Field, Result as PolarResult, Schema, DataFrame,};
use rust_ds::*;

fn main() {
    /* https://github.com/Steboss/ML_and_Rust/tree/master */
    config_env();
    let path = "/Users/barrychen/Desktop/Churn_Modelling.csv";
    let df = read_csv(&path).unwrap();
    let df2 = df.clone();
    let df3 = df.clone();
    let mut df_encode = df.clone();
    let mut df_encode2 = df.clone();

    // ////////////////////////////////////////////////////////////////////////////////////////// //
    /* show first several rows and header */
    // println!("{:?}", df.head(Some(5)));

    /* print column names */
    // let column_name = df.get_column_names(); 
    // println!("{:?}", &column_name);

    /* show shape */
    // let shape = df.shape();
    // println!("{:#?}", shape);

    /* print dataframe */
    // println!("{:?}", df);

    /* get row size and column size */
    // let row_len = df.height();
    // println!("number of rows: {:?}", row_len);
    // let col_len = df.width();
    // println!("number of columns: {:?}", col_len);

    /* loop using row/column size number */
    // for i in 0..col_len {
    //     println!("{:?}", i);
    // }
    
    /* select by column index */
    let column_name = df.get_column_names(); 
    let new_cols = col_select(&column_name, 3,  - 1);
    println!("{:?}", &new_cols);
    // let new_df = get_df_result(df.select(&new_cols)); // Select columns from the fourth column (index 3) up to the second-to-last column
    // println!("{:?}", new_df.head(Some(5)));
    // let new_df2 = new_df.slice(10, 5); // eqivalent to 10:15 rows in pandas
    // println!("{:?}", new_df2);

    // let new_df3 = pandas_iloc(&df, 3,  - 1, 10, 15);
    // println!("{:?}", new_df3);

    /* label encoding */
    let unique_gender2 = df.column("Gender").unwrap().unique().unwrap(); // extract unique values
    let unique_gender = get_sr_result(get_unique_values(&mut df_encode, "Gender"));
    println!("{:?}", unique_gender);

    let encode_col = get_encoded_column(unique_gender, "Gender", df).unwrap();
    let df_encode = df_encode.with_column(Series::new("Gender", encode_col)).unwrap();

    let check_encode = pandas_iloc(&df_encode, 3, -1, 10, 15);
    println!("{:?}", check_encode);

    /* want to do label encoding manually */
    println!("{:?}", unique_gender2); // first check what are the labels we want to encode
    let my_label_vec = vec![
        ("Female", 3),
        ("Male", 2)
    ]; // manual define the labels

    let encode_col2 = get_encoded_column_vec(my_label_vec, "Gender", df2).unwrap();
    let df_encode2 = df_encode2.with_column(Series::new("Gender", encode_col2)).unwrap();

    let check_encode2 = pandas_iloc(&df_encode2, 3, -1, 10, 15);
    println!("{:?}", check_encode2);

    /* let's one hot encoding! */
    let encode_geo_cols = one_hot_encoding(df3, "Geography");
    println!("{:?}", encode_geo_cols);

    
    

    /* 2D array */
    // println!("{:#?}", df.schema()); // check schema
    
    // let new_df = df
    // .clone()
    // .lazy()
    // .select([dtype_cols([DataType::Int64, DataType::Float64])])
    // .collect();

    // let num_df = get_df_result(new_df);

    // println!("{:?}", num_df);
    // println!("{:#?}", num_df.schema());

    // let my_array = num_df.to_ndarray::<Float32Type>(IndexOrder::Fortran).unwrap();
    // println!("{:?}", my_array);
    
    
    // println!("{:?}", num_df);
    // let my_array = df.to_ndarray(IndexOrder::Fortran).unwrap();

    /*  */

    /*  */

}

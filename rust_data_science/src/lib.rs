use polars::prelude::*;//{CsvReader, DataType, Field, Result as PolarResult, Schema, DataFrame,};
use polars::prelude::PolarsResult;
// use polars::prelude::DataType;
// use polars_core::prelude::*;
use polars::frame::DataFrame;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::env;
use polars::prelude::SerReader;

pub fn read_csv<P: AsRef<Path>>(path: P) -> PolarsResult<DataFrame> {
    /* Example function to create a dataframe from an input csv file*/
    let file = File::open(path).expect("Cannot open file.");

    CsvReader::new(file) 
    .has_header(true)
    .finish()
}

pub fn get_shape_info<P: AsRef<Path>>(path: P) -> () {
    /* Example function to retrieve shape info from a dataframe */
    let df = read_csv(&path).unwrap();
    // shape 
    // reming {:#?} otherwise error `(usize, usize)` cannot be formatted with the default formatter
    let shape = df.shape();
    println!("{:#?}", shape); 
    //schema
    println!("{:#?}", df.schema());
    //dtypes 
    println!("{:#?}", df.dtypes());
    //or width and height
    let width = df.width();
    println!("{}", width);
    let height = df.height();
    println!("{}", height);
}

pub fn get_column_info<P: AsRef<Path>>(path: P) -> () {
    /* Examples to deal with column and column names and enumerate */
    let df = read_csv(&path).unwrap();
    //column functions
    let columns = df.get_columns(); // you can do for column in columns{}
    let columname = df.get_column_names(); 

    // example like Python for i, val in enumerate(list, 0):
    for (i, column) in columns.iter().enumerate(){
        println!("{}, {}", column, columname[i]);
    }
}

pub fn get_df_result(new_df: Result<DataFrame, PolarsError>) -> DataFrame {
    match new_df {
        Ok(df) => df, // Return the DataFrame if it's Ok
        Err(e) => {
            eprintln!("Error: {:?}", e);
            DataFrame::default()
        },
    }
}

pub fn get_sr_result(new_sr: Result<Series, PolarsError>) -> Series {
    match new_sr {
        Ok(sr) => sr, // Return the Series if it's Ok
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Series::default()
        },
    }
}

pub fn config_env() {
    env::set_var("POLARS_FMT_MAX_COLS", "20"); // maximum number of columns shown when formatting DataFrames.
    // env::set_var("POLARS_FMT_MAX_ROWS", "10");   // maximum number of rows shown when formatting DataFrames.
    // env::set_var("POLARS_FMT_STR_LEN", "50");    // maximum number of characters printed per string value.
}

pub fn col_select<T: Clone>(items: &[T], start_idx: usize, mut end_idx: isize) -> Vec<T> {
    if end_idx < 0 {
        end_idx = items.len() as isize + end_idx;
    }
    items[start_idx..end_idx as usize].to_vec()
}

pub fn pandas_iloc(df: &DataFrame, col_start_idx: usize, mut col_end_idx: isize, row_start_idx: usize, row_end_idx: usize) -> DataFrame {
    // Select columns based on start and end indices
    let column_names = df.get_column_names();
    if col_end_idx < 0 {
        col_end_idx = column_names.len() as isize + col_end_idx;
    }
    let new_cols = col_select(&column_names, col_start_idx, col_end_idx as usize as isize);

    // Create a new DataFrame with selected columns
    let new_df = get_df_result(df.select(&new_cols));

    // Select rows based on start and end indices
    let new_df2 = new_df.slice(row_start_idx as i64, row_end_idx - row_start_idx);

    new_df2
}

pub fn get_unique_values(df: &mut DataFrame, column_name: &str) -> PolarsResult<Series> {
    // Get unique values of the column
    let unique_values = df
    .column(column_name).unwrap_or_else(|_| {
        panic!("Column {} not found in DataFrame", column_name)
    }).unique().unwrap();
    Ok(unique_values)
}

pub fn get_encoded_column(series: Series, column_name: &str, df: DataFrame)-> PolarsResult<Vec<u32>>{
    
    let mut mapping = HashMap::new();
    for (index, value) in series.str()?.into_iter().enumerate() {
        mapping.insert(value, index as u32);
    }
    print!("Mapping: {:?}", mapping);

    // Map the values in the column to integers
    Ok(df.column(column_name).unwrap()
    .str()?
    .into_iter()
    .map(|value| *mapping.get(&value).unwrap() as u32)
    .collect::<Vec<u32>>())
}

pub fn get_encoded_column_vec(label_vec: Vec<(&str,i32)>, column_name: &str, df: DataFrame)-> PolarsResult<Vec<u32>>{
    
    let mut mapping = HashMap::new();
    for (category, label) in label_vec {
        mapping.insert(Some(category), label as u32);
    }
    print!("Mapping: {:?}", mapping);

    // Map the values in the column to integers
    Ok(df.column(column_name).unwrap()
    .str()?
    .into_iter()
    .map(|value| *mapping.get(&value).unwrap() as u32)
    .collect::<Vec<u32>>())
}

pub fn any_to_string<'a>(v: AnyValue<'a>) -> String {
    match v {
        AnyValue::String(b) => b.to_string(),
        _ => format!("{:?}", v),
    }
}

pub fn one_hot_encoding(df: DataFrame, col_name: &str) -> DataFrame {
    
    let drop_cols_vec = df.get_column_names();
    
    let mut df_encode = df.clone();

    let cols = df.column(col_name).unwrap().unique().unwrap();

    let cols_vec: Vec<&str> = cols
        .iter()
        .map(|value| match value {
            AnyValue::String(s) => s,
            _ => panic!("Expected string value"),
        })
        .collect();

    for j in 0..cols_vec.len() {
        let mut the_vec = vec![0;df_encode.height()];
        for i in 0..df.height() {
            if any_to_string(df_encode.column(col_name).unwrap().get(i).unwrap()) == cols_vec[j].to_string() {
                the_vec[i] = 1;
            }
        }
        let the_new_series = Series::new(cols_vec[j], &the_vec);
        let _ = df_encode.with_column(the_new_series);
    }

    let columns_to_keep: Vec<&str> = df_encode
        .get_column_names()
        .iter()
        .filter(|&name| !drop_cols_vec.contains(name))
        .map(|&name| name)
        .collect();

    let df_encode = df_encode.select(columns_to_keep).unwrap();

    df_encode

}

pub fn any_to_float64<'a>(v: AnyValue<'a>) -> f64 {
    match v {
        AnyValue::String(b) => b.parse::<f64>().unwrap_or(0.0),
        AnyValue::Float64(f) => f,
        AnyValue::Int32(i) => i as f64,
        AnyValue::Int64(i) => i as f64,
        _ => 0.0, // Return a default value or handle other cases as needed
    }
}

pub fn standard_scaler(df_train: DataFrame, df_test: DataFrame) -> (DataFrame, DataFrame) {
    let mut df_train_sc = DataFrame::default();
    let mut df_test_sc = DataFrame::default();

    let n = df_train.height() as i32; // num of data points

    let mut df_train_t = df_train.transpose(None, None).unwrap();
    let col_mean = df_train_t.mean_horizontal(polars::frame::NullStrategy::Ignore).unwrap();
    let df_w_mean: &mut DataFrame;
    if let Some(mean) = col_mean {
        df_w_mean = df_train_t.with_column(mean.with_name("col_mean")).unwrap();
    } else {
        panic!("Mean not found for DataFrame");
    }

    let sse = df_w_mean
        .clone()
        .lazy()
        .with_column((col("*") - col("col_mean")).pow(2))
        .collect().unwrap()
        .sum_horizontal(polars::frame::NullStrategy::Ignore).unwrap();

    let df_w_sse: &mut DataFrame;
    if let Some(sse) = sse {
        df_w_sse = df_w_mean.with_column(sse.with_name("col_std")).unwrap();
    } else {
        panic!("SSE not found for DataFrame");
    }

    let result = df_w_sse.clone().lazy().with_column((col("col_std") / lit(n - 1)).sqrt()).collect().unwrap();

    let col_mean = result.column("col_mean").unwrap();
    let col_std = result.column("col_std").unwrap();

    let cols_vec = df_train.get_column_names();
    
    for j in 0..df_train.width() as usize {
        let cols = df_train.column(cols_vec[j]).unwrap();
        let mut the_vec_f64: Vec<_> = match cols.dtype() {
            DataType::Int32 => {
                cols.iter()
                    .map(|value| match value {
                        AnyValue::Int32(s) => s as f64,
                        _ => panic!("Expected value"),
                    })
                    .collect()
            }
            DataType::Float64 => {
                cols.iter()
                    .map(|value| match value {
                        AnyValue::Float64(s) => s,
                        _ => panic!("Expected value"),
                    })
                    .collect()
            }
            _ => panic!("Unsupported data type"),
        };

        for i in 0..df_train.height() {
            the_vec_f64[i] = (the_vec_f64[i] - any_to_float64(col_mean.get(j).unwrap()))/any_to_float64(col_std.get(j).unwrap());
        }
        let the_new_series = Series::new(cols_vec[j], &the_vec_f64);
        let _ = df_train_sc.with_column(the_new_series);
    }

    for j in 0..df_test.width() as usize {
        let cols = df_test.column(cols_vec[j]).unwrap();

        let mut the_vec_f64: Vec<_> = match cols.dtype() {
            DataType::Int32 => {
                cols.iter()
                    .map(|value| match value {
                        AnyValue::Int32(s) => s as f64,
                        _ => panic!("Expected value"),
                    })
                    .collect()
            }
            DataType::Float64 => {
                cols.iter()
                    .map(|value| match value {
                        AnyValue::Float64(s) => s,
                        _ => panic!("Expected value"),
                    })
                    .collect()
            }
            _ => panic!("Unsupported data type"),
        };

        for i in 0..df_test.height() {
            the_vec_f64[i] = (the_vec_f64[i] - any_to_float64(col_mean.get(j).unwrap()))/any_to_float64(col_std.get(j).unwrap());
        }
        let the_new_series = Series::new(cols_vec[j], &the_vec_f64);
        let _ = df_test_sc.with_column(the_new_series);
    }

    (df_train_sc, df_test_sc)
}

pub fn train_test_split(df: DataFrame, test_size: f64, is_shuffle: bool, seed: u64) -> (DataFrame, DataFrame) {

    let num_rows = df.height();
    let shuffled_df = df.sample_n_literal(num_rows,false,is_shuffle,Some(seed)).unwrap();
    let res = (num_rows as f64 * test_size).round() as usize;
    let df_train = shuffled_df.slice(0i64, num_rows - res);
    let df_test = shuffled_df.slice((num_rows - res) as i64, res);
    (df_train, df_test)

}

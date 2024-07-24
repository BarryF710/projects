use polars::prelude::*;
use polars::frame::DataFrame;
use rust_ds::{any_to_float64, standard_scaler};

fn main() {
    // Create two sample dataframe (training, test)
    let df_train = DataFrame::new(vec![
        Series::new("col_1", &[3.0, 3.0, 9.0, 9.0, 6.0]),
        Series::new("col_2", &[5, 10, 15, 10, 5]),
    ])
    .unwrap();

    let df_test = DataFrame::new(vec![
        Series::new("col_1", &[3.0, 6.0, 9.0]),
        Series::new("col_2", &[5, 10, 15]),
    ])
    .unwrap();

    let df_train2 = df_train.clone();
    let df_test2 = df_test.clone();

    let mut df_train_sc = DataFrame::default();
    let mut df_test_sc = DataFrame::default();

    // Standardization: z = (x - u)/s, where x is the sample data, u is the mean of the training samples
    // s is  the standard deviation of the training samples

    let n = df_train.height() as i32; // num of data points

    let mut df_train_t = df_train.transpose(None, None).unwrap();
    let col_mean = df_train_t.mean_horizontal(polars::frame::NullStrategy::Ignore).unwrap();
    let df_w_mean: &mut DataFrame;
    if let Some(mean) = col_mean {
        df_w_mean = df_train_t.with_column(mean.with_name("col_mean")).unwrap();
    } else {
        panic!("Mean not found for DataFrame");
    }

    // println!("{:?}", df_w_mean);

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
    // println!("{:?}", result);

    let col_mean = result.column("col_mean").unwrap();
    let col_std = result.column("col_std").unwrap();
    // println!("{:?}", col_mean.get(0).unwrap());
    // println!("{:?}", col_std);

    let cols_vec = df_train.get_column_names();

    // println!("num of cols: {:?}", df_train.width());
    // println!("num of data points: {:?}", df_train.height());
    // let cols = df_train.column(cols_vec[0]).unwrap();
    // println!("{:?}", cols);

    // println!("the vec:");
    // let the_vec: Vec<i32> = cols
    //     .iter()
    //     .map(|value| match value {
    //         AnyValue::Int32(s) => s,
    //         _ => panic!("Expected value"),
    //     })
    //     .collect();

    // let the_vec_f64: Vec<f64> = the_vec.iter().map(|&x| x as f64).collect();
    // println!("{:?}", the_vec_f64);
    
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

        // let mut the_vec_f64: Vec<f64> = the_vec.iter().map(|&x| x as f64).collect();

        for i in 0..df_train.height() {
            the_vec_f64[i] = (the_vec_f64[i] - any_to_float64(col_mean.get(j).unwrap()))/any_to_float64(col_std.get(j).unwrap());
        }
        let the_new_series = Series::new(cols_vec[j], &the_vec_f64);
        let _ = df_train_sc.with_column(the_new_series);
    }

    println!("{:?}", df_train_sc);

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

        // let the_vec: Vec<i32> = cols
        //     .iter()
        //     .map(|value| match value {
        //         AnyValue::Int32(s) => s,
        //         _ => panic!("Expected value"),
        //     })
        //     .collect();

        // let the_vec: Vec<f64> = cols
        //     .iter()
        //     .map(|value| match value {
        //         AnyValue::Float64(s) => s,
        //         _ => panic!("Expected value"),
        //     })
        //     .collect();


        // let mut the_vec_f64: Vec<f64> = the_vec.iter().map(|&x| x as f64).collect();

        for i in 0..df_test.height() {
            the_vec_f64[i] = (the_vec_f64[i] - any_to_float64(col_mean.get(j).unwrap()))/any_to_float64(col_std.get(j).unwrap());
        }
        let the_new_series = Series::new(cols_vec[j], &the_vec_f64);
        let _ = df_test_sc.with_column(the_new_series);
    }

    println!("{:?}", df_test_sc);

    let (df_train_sc2, df_test_sc2) = standard_scaler(df_train2, df_test2);
    println!("{:?}", df_train_sc2);
    println!("{:?}", df_test_sc2);

}
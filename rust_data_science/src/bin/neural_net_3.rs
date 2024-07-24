#[allow(unused_imports)]
use rust_ds::*;
use anyhow::{Ok, Result};
use tch::{nn, nn::Module, nn::OptimizerConfig, Device};
use polars::prelude::*;
use tch::{Tensor, Kind, vision, Scalar};

fn dataframe_to_tensor(df: DataFrame) -> Tensor {
    // Get number of rows and columns
    let num_rows = df.height();
    let num_cols = df.width();

    // Create a vector to store the data
    let mut data_vector: Vec<f32> = Vec::with_capacity(num_rows * num_cols);

    // Iterate over the rows of the DataFrame
    for i in 0..num_rows {
        // Iterate over the column names of the DataFrame
        for col_name in df.get_column_names() {
            // Get the index of the column by name
            // let j = df.get_column_index(col_name).unwrap();
            // let mut j = col_name;

            // Get the value at the current row and column
            let value = match df.column(col_name).unwrap().get(i).unwrap() {
                AnyValue::Float64(v) => v as f32,
                AnyValue::Int64(v) => v as f32,
                // None => panic!("Encountered None value"),
                _ => panic!("Unsupported data type"),
            };

            // Push the value into the data vector
            data_vector.push(value);
        }
    }

    // Initialize tensor directly from the data vector
    let tensor = Tensor::f_from_slice(&data_vector)
        .expect("Failed to create tensor from slice")
        .reshape(&[num_rows as i64, num_cols as i64])
        .to(Device::Cpu);

    tensor
}

const INPUT: i64 = 12; // 784
const HIDDEN1: i64 = 8; // 128
const HIDDEN2: i64 = 4; // 128
const OUTPUT: i64 = 2; // 10

fn net(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::linear(vs / "layer1", INPUT, HIDDEN1, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs / "layer2", HIDDEN1, HIDDEN2, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs, HIDDEN2, OUTPUT, Default::default()))
}

fn run() -> Result<()> {
    let m = tch::vision::mnist::load_dir("/Users/barrychen/Downloads/MNIST_ORG")?;
    let vs = nn::VarStore::new(Device::Cpu);
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
    for epoch in 1..200 {
        let loss = net.forward(&m.train_images).cross_entropy_for_logits(&m.train_labels);
        opt.backward_step(&loss);
        let test_accuracy = net.forward(&m.test_images).accuracy_for_logits(&m.test_labels);
        println!(
            "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
            epoch,
            f64::try_from(&loss)?,
            100. * f64::try_from(&test_accuracy)?,
        );
    }


    Ok(())
}

fn run2() -> Result<()> {
    let df = get_df_result(read_csv("/Users/barrychen/Downloads/Xy_data.csv"));
    // let x_df = pandas_iloc(&df, 0, 12, 0, df.height());
    // println!("X df shape: {:?}", x_df.shape());
    // println!("{:?}", x_df.head(Some(5)));
    // let y_df = pandas_iloc(&df, 12, 13, 0, df.height());
    // println!("y df shape: {:?}", y_df.shape());
    // println!("{:?}", y_df.head(Some(5)));

    let (df_train, df_test) = train_test_split(df, 0.2, false, 1);
    // println!("training df shape: {:?}", df_train.shape());
    // println!("testing df shape: {:?}", df_test.shape());

    let x_df_train = pandas_iloc(&df_train, 0, 12, 0, df_train.height());
    let y_df_train = pandas_iloc(&df_train, 12, 13, 0, df_train.height());
    let x_df_test = pandas_iloc(&df_test, 0, 12, 0, df_test.height());
    let y_df_test = pandas_iloc(&df_test, 12, 13, 0, df_test.height());
    // println!("training X df shape: {:?}", x_df_train.shape());
    // println!("training y df shape: {:?}", y_df_train.shape());
    // println!("testing X df shape: {:?}", x_df_test.shape());
    // println!("testing y df shape: {:?}", y_df_test.shape());

    let x_ts_train = dataframe_to_tensor(x_df_train);
    let y_ts_train = dataframe_to_tensor(y_df_train).squeeze().to_kind(Kind::Int64);
    let x_ts_test = dataframe_to_tensor(x_df_test);
    let y_ts_test = dataframe_to_tensor(y_df_test).squeeze().to_kind(Kind::Int64);
    // println!("training X tensor shape: {:?}", x_ts_train.size());
    // println!("training y tensor shape: {:?}", y_ts_train.size());
    // println!("testing X tensor shape: {:?}", x_ts_test.size());
    // println!("testing y tensor shape: {:?}", y_ts_test.size());

    let vs = nn::VarStore::new(Device::Cpu);
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
    for epoch in 1..100 {
        let loss = net.forward(&x_ts_train).cross_entropy_for_logits(&y_ts_train);
        opt.backward_step(&loss);
        let test_accuracy = net.forward(&x_ts_test).accuracy_for_logits(&y_ts_test);
        println!(
            "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
            epoch,
            f64::try_from(&loss)?,
            100. * f64::try_from(&test_accuracy)?,
        );
    }
    Ok(())
}

fn main() {
    // let _ = run();
    let _ = run2();
}
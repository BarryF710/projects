#[allow(unused_imports)]
use tch::{Tensor, Kind, Device, vision, Scalar};
use rust_ds::*;
use polars::prelude::*;
use std::collections::HashMap;
// use std::any::type_name;

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

//Define a linear neural network layer
trait Compute {
    fn forward (&self,  mem: &Memory, input: &Tensor) -> Tensor;
}

struct Linear {
    params: HashMap<String, usize>,
}

impl Linear {
    fn new (mem: &mut Memory, ninputs: i64, noutputs: i64) -> Self {
        let mut p = HashMap::new();
        p.insert("W".to_string(), mem.new_push(&[ninputs,noutputs], true));
        p.insert("b".to_string(), mem.new_push(&[1, noutputs], true));

        Self {
            params: p,
        }
    } 
}

impl Compute for Linear {
    fn forward (&self,  mem: &Memory, input: &Tensor) -> Tensor {
        let w = mem.get(self.params.get(&"W".to_string()).unwrap());
        let b = mem.get(self.params.get(&"b".to_string()).unwrap());
        input.matmul(w) + b
    }
}

//Define neural network model
struct MyModel {
    l1: Linear,
    l2: Linear,
    l3: Linear,
}

impl MyModel {
    fn new (mem: &mut Memory) -> MyModel {
        let l1 = Linear::new(mem, 12, 8);
        let l2 = Linear::new(mem, 8, 4);
        let l3 = Linear::new(mem, 4, 2);
        Self {
            l1,
            l2,
            l3,
        }
    }
}

impl Compute for MyModel {
    fn forward (&self,  mem: &Memory, input: &Tensor) -> Tensor {
        let mut o = self.l1.forward(mem, input);
        o = o.relu();
        o = self.l2.forward(mem, &o);
        o = o.relu();
        o = self.l3.forward(mem, &o);
        o.sigmoid()
    }
}

//Mean Squared Error and Cross Entropy Loss functions
fn mse(target: &Tensor, pred: &Tensor) -> Tensor {
    (target - pred).square().mean(Kind::Float)
}

fn cross_entropy (target: &Tensor, pred: &Tensor) -> Tensor {
    //Print tensor information
    // let target_size = target.size();
    // let tnum_dims = target.dim();
    // println!("Target size: {:?}", target_size);
    // println!("Number of target dimensions: {}", tnum_dims);
    // let pred_size = pred.size();
    // let pnum_dims = pred.dim();
    // println!("Pred size: {:?}", target_size);
    // println!("Number of pred dimensions: {}", pnum_dims);

    let loss = pred.log_softmax(1, Kind::Float).nll_loss(target);
    loss
}

//Memory
struct Memory {
    size: usize,
    values: Vec<Tensor>,
}

impl Memory {

    fn new() -> Self {
        let v = Vec::new();
        Self {size: 0,
            values: v}
    }

    fn push (&mut self, value: Tensor) -> usize {
        self.values.push(value);
        self.size += 1;
        self.size-1
    }

    fn new_push (&mut self, size: &[i64], requires_grad: bool) -> usize {
        let t = Tensor::randn(size, (Kind::Float, Device::Cpu)).requires_grad_(requires_grad);
        self.push(t)
    }

    fn get (&self, addr: &usize) -> &Tensor {
        &self.values[*addr]
    }

    fn apply_grads_sgd(&mut self, learning_rate: f32) {
        let mut g = Tensor::new();      
        self.values
        .iter_mut()
        .for_each(|t| {
            if t.requires_grad() {
                g = t.grad();
                t.set_data(&(t.data() - learning_rate*&g));
                t.zero_grad();
            }
        });
    }

    fn apply_grads_sgd_momentum(&mut self, learning_rate: f32) {
        let mut g: Tensor = Tensor::new();
        let mut velocity: Vec<Tensor>= Tensor::zeros(&[self.size as i64], (Kind::Float, Device::Cpu)).split(1, 0);
        let mut vcounter = 0;
        const BETA:f32 = 0.9;
        
        self.values
        .iter_mut()
        .for_each(|t| {
            if t.requires_grad() {
                g = t.grad();
                velocity[vcounter] = BETA * &velocity[vcounter] + (1.0 - BETA) * &g;
                t.set_data(&(t.data() - learning_rate * &velocity[vcounter]));
                t.zero_grad();
            }
            vcounter += 1;
        });
    }
}

//Training function
fn train<F>(mem: &mut Memory, x: &Tensor, y: &Tensor, model: &dyn Compute, epochs: i64, batch_size: i64, errfunc: F, learning_rate: f32) 
    where F: Fn(&Tensor, &Tensor)-> Tensor    
        {
        let mut error = Tensor::from(0.0);
        let mut batch_error = Tensor::from(0.0);
        let mut pred = Tensor::from(0.0);
        for epoch in 0..epochs {
            batch_error = Tensor::from(0.0);
            for (batchx, batchy) in get_batches(&x, &y, batch_size, true) {
                pred = model.forward(mem, &batchx);
                error = errfunc(&batchy, &pred);
                batch_error += error.detach();
                error.backward();
                mem.apply_grads_sgd_momentum(learning_rate);              
            }
            println!("Epoch: {:?} Error: {:?}", epoch, batch_error/batch_size);
        }
}

//Mini-batch
fn get_batches(x: &Tensor, y: &Tensor, batch_size: i64, shuffle: bool) -> impl Iterator<Item = (Tensor, Tensor)> {
    let num_rows = x.size()[0];
    let num_batches = (num_rows + batch_size - 1) / batch_size;
    
    let indices = if shuffle {
        Tensor::randperm(num_rows as i64, (Kind::Int64, Device::Cpu))
    } else 
    {
        let rng = (0..num_rows).collect::<Vec<i64>>();
        Tensor::from_slice(&rng)
    };
    let x = x.index_select(0, &indices);
    let y = y.index_select(0, &indices);
    
    (0..num_batches).map(move |i| {
        let start = i * batch_size;
        let end = (start + batch_size).min(num_rows);
        let batchx: Tensor = x.narrow(0, start, end - start);
        let batchy: Tensor = y.narrow(0, start, end - start);
        (batchx, batchy)
    })
}

fn accuracy(target: &Tensor, pred: &Tensor) -> f64 {
    let yhat = pred.argmax(1,true).squeeze();
    let eq = target.eq_tensor(&yhat);
    let accuracy: f64 = (eq.sum(Kind::Int64) / target.size()[0]).double_value(&[]).into();
    accuracy
}

// fn load_mnist() -> (Tensor, Tensor) {
//     let m = vision::mnist::load_dir("/Users/barrychen/Downloads/MNIST_ORG").unwrap();
//     let x = m.train_images;
//     let y = m.train_labels;
//     (x, y)
// }

#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    let x_df = get_df_result(read_csv("/Users/barrychen/Downloads/X_data.csv"));
    // println!("{:?}", x_df);
    let y_df = get_df_result(read_csv("/Users/barrychen/Downloads/y_data.csv"));
    // println!("{:?}", y_df);
    
    // let column_types = x_df.dtypes(); // Get the data types of the columns
    // println!("{:?}", column_types);
    let x_tensor = dataframe_to_tensor(x_df);
    println!("Shape of tensor: {:?}", x_tensor.size());

    let y_tensor = dataframe_to_tensor(y_df).squeeze().to_kind(Kind::Int64);
    println!("Shape of tensor: {:?}", y_tensor.size());
    
    // &x_tensor.slice(0, 0, 5 as i64, 1).print(); // Print the first few rows of the tensor

    // let (x, y) = load_mnist();
    
    // println!("Shape of x: {:?}", x.size()); // Print the shape of x
    // println!("Shape of y: {:?}", y.size()); // Print the shape of y
    // let num_rows_to_display = 5; // Display a few rows of x
    // let rows_to_display = x.narrow(0, 0, num_rows_to_display);
    // println!("First {} rows of x:\n{}", num_rows_to_display, rows_to_display);

    // println!("Shape of x: {:?}", &x_tensor.size()); // Print the shape of x
    // println!("Shape of y: {:?}", y.size()); // Print the shape of y

    let mut m = Memory::new();
    let mymodel = MyModel::new(&mut m);
    train(&mut m, &x_tensor, &y_tensor, &mymodel, 100, 32, cross_entropy, 0.3);
    let out = mymodel.forward(&m, &x_tensor);
    println!("Training Accuracy: {}", accuracy(&y_tensor, &out));
}
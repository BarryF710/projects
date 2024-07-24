use polars::prelude::*;
use rust_ds::any_to_string;

fn main() {
    let df = DataFrame::new(vec![
        Series::new("location", &["A", "B", "C", "A", "B"]),
        Series::new("value", &[1, 2, 3, 4, 5]),
    ])
    .unwrap();

    let mut df_encode = df.clone();

    // let loc_sr = Series::new("location", vec!["A","B","C","A","B"]);
    // println!("{:?}", sr);

    let cols_vec = vec!["A","B","C"];

    // Iterate over the indices
    // let my_check = any_to_str(df_encode.column("location").unwrap().get(1).unwrap());
    for j in 0..cols_vec.len() {
        let mut the_vec = vec![0;5];
        for i in 0..df.height() {
            if any_to_string(df_encode.column("location").unwrap().get(i).unwrap()) == cols_vec[j].to_string() {
                the_vec[i] = 1;
            }
        }
        let the_new_series = Series::new(cols_vec[j], &the_vec);
        let _ = df_encode.with_column(the_new_series);
    }


    // for i in 0..df.height() {
    //     if any_to_str(df_encode.column("location").unwrap().get(i).unwrap()) == "A".to_string() {
    //         the_vec[i] = 1;
    //     }
    // }

    

    // let the_new_series = Series::new("A", &the_vec);

    // println!("one hot for A: \n{:?}", &the_new_series);

    // let _ = df_encode.with_column(the_new_series);

    println!("{:?}", &df_encode);

    
}
use polars::prelude::*;

fn read_csv_to_dataframe(path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    // Read CSV file
    let df_csv = CsvReader::from_path(path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    Ok(df_csv)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let mut df: DataFrame = df!(
    //     "integer" => &[1, 2, 3],
    //     "week" => &["Mon","Tue","Wed"],
    //     "float" => &[4.0, 5.0, 6.0]
    // )
    // .unwrap();
    // println!("{}", df);

    //Provide the file path
    let file_path = "/Users/barrychen/Downloads/ind_ban_comment.csv";
    
    //Call the function to read CSV into DataFrame
    let df = read_csv_to_dataframe(file_path)?;

    //Print the DataFrame
    // println!("{:?}", df);

    //Output
    let out = df.clone().lazy().select([col("Batsman"), col("Bowler")]).collect()?;
    println!("{}", out);

    Ok(())  
}
use std::{fs, io};
use std::path::PathBuf;
use std::collections::BTreeMap;

const EXTENSION_TO_SORT: &str = ".json";


pub fn sort_year(base_path: &str, year: &str) -> Result<(), io::Error> {
    let composed_path = [base_path, year];
    let year_dir: PathBuf = composed_path.iter().collect();

    for entry in fs::read_dir(year_dir)? {
        let entry = entry?;
        let month_path = entry.path();

        let month_filename;
        match month_path.file_name() {
            Some(s) => month_filename = s,
            None => panic!("No filename for the given month!")
        } ;
        println!(
            "Sorting: {}-{}",
            month_filename.to_str().unwrap(),
            year,
        );
        sort_month(month_path)?;
    }
    Ok(())
}

fn sort_month(month_path: PathBuf) -> Result<(), io::Error> {
    let mut files_sorted_by_ts = BTreeMap::new();

    // Retrieve all the files
    for entry in fs::read_dir(month_path)? {
        let entry = entry?;
        let file_path = entry.path();
        let ts = file_path_to_timestamp(&file_path);
        files_sorted_by_ts.insert(ts, file_path);
    }

    // Rewrite the filename according to the order
    let mut i: u64 = 0;
    for (_timestamp, file_path) in &files_sorted_by_ts {
        let new_file_name = format!("{}{}", i, EXTENSION_TO_SORT);
        let mut new_file_path = file_path.clone();
        new_file_path.set_file_name(new_file_name);
        fs::rename(file_path, new_file_path)?;
        i += 1;
    }
    Ok(())
}

fn file_path_to_timestamp(file_path: &PathBuf) -> u64 {
    let filename = file_path.file_name().unwrap();
    let str_filename = filename.to_str().unwrap();
    assert!(str_filename.ends_with(EXTENSION_TO_SORT));
    let mut ts = str_filename.trim_end_matches(EXTENSION_TO_SORT);
    ts = ts.split("_").last().unwrap();
    ts.parse().unwrap()
}


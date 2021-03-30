use std::{fs, io, thread};
use std::path::{PathBuf, Path};
use std::collections::BTreeMap;

const EXTENSION_TO_SORT: &str = ".json";


pub fn sort_year(base_path: &str, year: &str) -> Result<(), io::Error> {
    let composed_path = [base_path, year];
    let year_dir: PathBuf = composed_path.iter().collect();

    let mut handles = Vec::new();  // used to wait for all threads to finish

    for entry in fs::read_dir(year_dir)? {
        let entry = entry?;
        let month_path = entry.path();

        let month_filename;
        match month_path.file_name() {
            Some(s) => month_filename = s,
            None => panic!("No filename for the given month!")
        };
        println!(
            "Sorting: {}-{}",
            month_filename.to_str().unwrap(),
            year,
        );
        let handle = thread::spawn(move || {
            let res = sort_month(&month_path);
            if let Err(e) = res {
                panic!("month {:?} wasn't processed correctly: {:?}", month_path, e)
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

fn sort_month(month_path: &Path) -> Result<(), io::Error> {
    let mut files_sorted_by_ts = BTreeMap::new();

    // Retrieve all the files
    for entry in fs::read_dir(month_path)? {
        let entry = entry?;
        let file_path = entry.path();
        let timestamp = file_path_to_timestamp(&file_path);
        match files_sorted_by_ts.get_mut(&timestamp) {
            None => {
                // No file exist for this timestamp, create a list of files for this given timestamp
                files_sorted_by_ts.insert(timestamp, vec![file_path]);
            }
            Some(existing_files) => {
                existing_files.push(file_path);
            }
        }
    }

    // Rewrite the filename according to the timestamp (BTreeMaps are sorted by keys)
    let mut i: u64 = 0;
    for files in files_sorted_by_ts.values() {
        // Iterate over all the file of a given timestamp (in no particular order)
        for file_path in files {
            let new_file_name = format!("{}{}", i, EXTENSION_TO_SORT);
            let mut new_file_path = file_path.clone();
            new_file_path.set_file_name(new_file_name);
            fs::rename(file_path, new_file_path)?;
            i += 1;
        }
    }
    Ok(())
}

fn file_path_to_timestamp(file_path: &Path) -> u64 {
    let filename = file_path.file_name().unwrap();
    let str_filename = filename.to_str().unwrap();
    assert!(str_filename.ends_with(EXTENSION_TO_SORT));
    let mut ts = str_filename.trim_end_matches(EXTENSION_TO_SORT);
    ts = ts.split('_').last().unwrap();
    ts.parse().unwrap()
}


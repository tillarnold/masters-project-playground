use std::{path::PathBuf, fs::File, io::BufReader, env};

/// Path to the example-data directory in the repo root
pub fn data_dir() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut par = PathBuf::from(manifest_dir);
    par.pop();
    par.push("example-data");

    par
}

/// Path to the `teacher_surivey.cvs` file
pub fn teacher_data_csv_file() -> PathBuf {
    let mut dir = data_dir();
    dir.push("teacher_survey.csv");
    dir
}

/// Columns of `teacher_surivey.cvs` as they are in the json file
pub fn teacher_data_csv_headers() -> Vec<&'static str> {
    return vec![
        "name",
        "sex",
        "age",
        "maritalStatus",
        "hasChildren",
        "highestEducationLevel",
        "sourceOfStress",
        "smoker",
        "optimism",
        "lifeSatisfaction",
        "selfEsteem",
    ];
}

/// Read `teacher_surivey.cvs` from disk and return the contents
/// 
/// This panics if anything goes wrong
pub fn read_teacher_data() -> Vec<Vec<String>> {
    const LIMIT: usize = 100;

    let file = File::open(teacher_data_csv_file()).expect("could not open csv file");
    let reader = BufReader::new(file);

    let mut rdr = csv::Reader::from_reader(reader);

    rdr.records()
        .take(LIMIT)
        .map(|rec| {
            rec.unwrap()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

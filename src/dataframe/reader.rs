extern crate csv;
extern crate rustc_serialize;

#[derive(Debug)]
pub struct StringDataFrame {
    pub features: Vec<Vec<String>>,
}

impl StringDataFrame {
    pub fn new() -> Self {
        StringDataFrame {
            features: Vec::new(),
        }
    }

    pub fn read_csv(filepath: &str, has_headers: bool, feature_dim: usize) -> StringDataFrame {
        let file = std::fs::File::open(filepath).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);

        let mut data_frame = StringDataFrame::new();

        // push all the records
        for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record, feature_dim);
        }
        return data_frame;
    }

    fn push(&mut self, row: &csv::StringRecord, feature_dim: usize) {
        let mut row_vec = Vec::with_capacity(feature_dim);
        for index in 0..feature_dim {
            row_vec.push(row[index].parse().unwrap());
        }
        self.features.push(row_vec);
    }
}

#[cfg(test)]
mod tests {
    use super::StringDataFrame;

    #[test]
    fn test_dataframe() {
        let has_headers = true;
        let feature_dim = 10;
        let filepath = "/Users/qtk/work/python/titanic-data/titanic_categorical.csv";
        let df = StringDataFrame::read_csv(filepath, has_headers, feature_dim);
        println!("{:?}", df);
    }
}

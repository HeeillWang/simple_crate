use crate::core::query::Query;
use crate::error::Error;

pub fn get_data(intput_file: String) -> Result<Vec<Query>, Error> {
    let mut ret = Vec::new();

    let mut rdr = csv::Reader::from_path(intput_file).unwrap();

    for row in rdr.deserialize() {
        let query: Query = match row {
            Ok(q) => q,
            Err(e) => {
                eprintln!("error on get_data() : {:?}", e);
                continue;
            }
        };
        ret.push(query);
    }

    Ok(ret)
}

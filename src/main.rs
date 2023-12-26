use std::error::Error;

mod nlx_csc;
use nlx_csc::*;

mod utilities;


fn main() -> Result<(), Box<dyn Error>>{
    let path = "/Users/bruskajp/Downloads/csc_data/LA2.ncs";

    let nlx_csc_file = NlxCscFile::open(path, None)?;

    let (_, iterator) = NlxCscFileIterator::new(path, None)?;
    for (i, record) in iterator.enumerate() {
        assert!(record? == nlx_csc_file.records[i]);
    }


    Ok(())
}



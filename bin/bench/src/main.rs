use csv_entry_storage::CSVEntryStorage;
use domain::core::entry::*;
use std::time::Instant;

fn main() {
    /*let entry = Entry::new(InformationAccess::new(
    ));*/
    println!("Hello, world!");
    //println!("{:#?}", entry);

    let now = Instant::now();
    let storage = CSVEntryStorage::new(
        "D:\\DEV\\GIT_PROJECTS\\design4green-api\\resources\\full.csv".to_string(),
    );

    println!(
        "Duration : {} seconds and {} nanoseconds",
        now.elapsed().as_secs(),
        now.elapsed().subsec_nanos()
    );

    storage.get();
}

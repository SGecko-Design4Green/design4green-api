use csv_entry_storage::CSVEntryStorage;
use domain::core::entry::*;

fn main() {
    /*let entry = Entry::new(InformationAccess::new(
    ));*/
    println!("Hello, world!");
    //println!("{:#?}", entry);

    let storage = CSVEntryStorage::new(
        "D:\\DEV\\GIT_PROJECTS\\design4green-api\\resources\\dataset_ARRAS.csv".to_string(),
    );

    storage.get();
}

use csv_entry_storage::CSVEntryStorage;
use domain::core::entry::*;
use std::time::Instant;

fn main() {
    /*let entry = Entry::new(InformationAccess::new(
    ));*/
    println!("Hello, world!");
    //println!("{:#?}", entry);

    let now = Instant::now();
    let mut storage = CSVEntryStorage::new(
        "D:\\DEV\\GIT_PROJECTS\\design4green-api\\resources\\full.csv".to_string(),
    );

    &storage.load();

    //
    //TO STOCK IN DB
    let dep = &storage.get_departments();
    println!("DEP >> Lines {:?}", dep.len());

    let reg = &storage.get_regions();
    println!("REG >> Lines {:?}", reg.len());

    let com = &storage.get_coms();
    println!("COM >> Lines {:?}", com.len());

    let res = &storage.get_entries();
    println!("ENTRY >> Lines {:?}", res.len());

    let reg_iris = &storage.get_regions_with_iris();
    println!("REG_IRIS >> Lines {:?}", reg_iris.len());

    let dep_iris = &storage.get_departements_with_iris();
    println!("DEP_IRIS >> Lines {:?}", dep_iris.len());
    println!("{:?}", dep_iris);

    println!(
        "Duration : {} seconds and {} nanoseconds",
        now.elapsed().as_secs(),
        now.elapsed().subsec_nanos()
    );
}

use std::fs::File;

File::open(s).unwrap_or_else(|_| {
    println!("File not found: {}, creating it...", s);
    File::create(s).unwrap()
})

#[test]
fn test_open_file() {
    let filename = "not_here.txt";

    if std::fs::metadata(filename).is_ok() {
        std::fs::remove_file(filename).unwrap();
    }

    open_file(filename);

    assert!(std::fs::metadata(filename).is_ok());

    std::fs::remove_file(filename).unwrap();
}



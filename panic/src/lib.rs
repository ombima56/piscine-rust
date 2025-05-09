use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}

#[test]
fn test_open_file() {
    let filename = "not_here.txt";

    if std::fs::metadata(filename).is_ok() {
        std::fs::remove_file(filename).unwrap();
    }

    let result = std::panic::catch_unwind(|| {
        open_file(filename);
    });

    assert!(result.is_err());
}


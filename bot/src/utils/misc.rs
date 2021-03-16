use std::fs::File;
use std::io::Read;

pub fn get_file_content(file_name: &str) -> Result<String, ()> {
    let mut file = match File::open(file_name) {
        Ok(ok) => ok,
        Err(_) => {
            return Err(());
        }
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => return Err(()),
    };
    Ok(content)
}

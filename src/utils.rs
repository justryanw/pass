use std::{path::PathBuf, fs::File};

use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;
use gtk::glib;

use crate::{APP_ID, login_object::LoginData};

pub fn data_path() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(APP_ID);
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("database");
    path
}

pub fn database_exists() -> bool {
    File::open(data_path()).is_ok()
}

pub fn write_database(data: Vec<LoginData>, password: &String) {
    let cocoon = Cocoon::new(password.as_bytes());

    let encoded = data.try_to_vec().unwrap();
    let mut file = File::create(data_path()).unwrap();
    cocoon.dump(encoded, &mut file).unwrap();
}

pub fn read_database(password: &String) -> Result<Vec<LoginData>, cocoon::Error> {
    let cocoon = Cocoon::new(password.as_bytes());

    let mut file = File::open(data_path()).unwrap();
    let read = cocoon.parse(&mut file)?;
    Ok(Vec::<LoginData>::try_from_slice(&read).unwrap())
}

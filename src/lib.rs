#[macro_use]
extern crate diesel;

pub mod schema;

use diesel::*;
use schema::User;

table! {
    users {
        id -> Serial,
        name -> VarChar,
        favorite_color -> Nullable<VarChar>,
    }
}

enum MyErr {
  Failed
}

fn users_with_name(connection: &Connection, target_name: &str)
  -> Option<Vec<(i32, String, Option<String>)>>
{
    use self::users::dsl::*;
    match users.filter(name.eq(target_name)).load(connection) {
      Ok(x) => Some(x.collect()),
      Err(_) => None
    }
}

#[test]
fn tests() {
  let url = "postgresql://localhost/rust";
  let connection = Connection::establish(url).unwrap();
  println!("Users:");
  for u in users_with_name(&connection, "name").unwrap().iter() {
    println!("{:?}", u);
  }
}

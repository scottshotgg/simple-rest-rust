use crate::schema::users;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub age: Option<i32>,
  // pub address: Address,
  pub hobby: Option<String>,
  pub email: Option<String>,
  // pub phone_1: Option<String>,
  // // pub phone_1_type: Option<PhoneKind>,
  // pub phone_2: Option<String>,
  // // pub phone_2_type: Option<PhoneKind>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Address {
  // pub kind: AddressKind,
  pub line_1: String,
  pub line_2: String,
  pub city: String,
  pub state: String,
  pub country: String,
  pub zipcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PhoneKind {
  Home,
  Cell,
  Business,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AddressKind {
  Residential,
  Commercial,
  Government,
}

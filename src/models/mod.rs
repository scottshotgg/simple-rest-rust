#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub age: i32,
  // pub address: Address,
  pub hobby: String,
  pub email: String,
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

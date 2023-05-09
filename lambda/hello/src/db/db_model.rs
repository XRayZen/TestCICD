use typed_builder::TypedBuilder;

#[derive(Default, TypedBuilder)]
pub struct DbModel {
    #[builder(default)]
    pub user_id: String,
    #[builder(default)]
    pub user_name: String,
    #[builder(default)]
    pub user_age: u32,
    #[builder(default)]
    pub user_address: String,
}

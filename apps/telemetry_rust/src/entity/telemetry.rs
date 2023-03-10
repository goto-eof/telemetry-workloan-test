use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "telemetry_rust")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub request_id: i32,
    pub code: String,
    pub property: String,
    pub value: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

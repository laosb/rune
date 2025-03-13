//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media_file_fingerprint")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub media_file_id: i32,
    #[sea_orm(column_type = "Blob")]
    pub fingerprint: Vec<u8>,
    pub is_duplicated: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::media_files::Entity",
        from = "Column::MediaFileId",
        to = "super::media_files::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MediaFiles,
}

impl Related<super::media_files::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaFiles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

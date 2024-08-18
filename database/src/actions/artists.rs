use std::collections::HashSet;

use sea_orm::prelude::*;

use crate::entities::media_cover_art;
use crate::entities::{artists, media_file_artists, media_files};
use crate::get_entity_to_cover_ids;
use crate::get_groups;

use super::utils::CountByFirstLetter;

impl CountByFirstLetter for artists::Entity {
    fn group_column() -> Self::Column {
        artists::Column::Group
    }

    fn id_column() -> Self::Column {
        artists::Column::Id
    }
}

get_groups!(
    get_artists_groups,
    artists,
    media_file_artists::Entity,
    media_file_artists::Column
);
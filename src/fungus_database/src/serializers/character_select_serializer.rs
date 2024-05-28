use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::serializers::avatar_look_serializer::AvatarLookSerializer;
use crate::serializers::character_serializer::CharacterSerializer;
use crate::serializers::character_stats_serializer::CharacterStatsSerializer;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CharacterSelectSerializer {
    pub character: CharacterSerializer,
    pub avatar_look: AvatarLookSerializer,
    pub character_stats: CharacterStatsSerializer
}
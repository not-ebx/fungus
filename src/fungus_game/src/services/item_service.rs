use sqlx::{Error, Postgres, Transaction};
use fungus_database::serializers::equipment_serializer::EquipmentSerializer;
use fungus_database::serializers::item_serializer::ItemSerializer;




pub struct ItemService {

}



impl ItemService {
    pub fn new() -> Self {
        ItemService {}
    }
}
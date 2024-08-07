use std::collections::HashSet;
use std::fmt::Error;
use fungus_utils::enums::inv_type::InvType;
use crate::entities::equipment::Equipment;
use crate::entities::item::Item;
use crate::errors::inventory_errors::InventoryOperationError;

pub trait InventoryOperations<T> {
    fn add(&mut self, item: T) -> Result<(), InventoryOperationError>;
    fn remove_by_slot(&mut self, slot: i16) -> Result<(), InventoryOperationError>;
    fn update_item(&mut self, item: T, qty: i32) -> Result<(), InventoryOperationError>;
    fn get(&self, slot: i16) -> Result<T, InventoryOperationError>;
    fn sort(&mut self) -> Result<T, InventoryOperationError>;
    fn collapse(&mut self) -> Result<T, InventoryOperationError>;
}

pub struct Inventory<T> {
    pub id: i64,
    pub slots: i16,
    pub inv_type: InvType,
    items: Vec<Option<T>>,
    unique_items: HashSet<i32>
}

impl InventoryOperations<Equipment> for Inventory<Equipment> {
    fn add(&mut self, equipment: Equipment) -> Result<(), InventoryOperationError> {
        // Equipments dont stack.
        if self.items.len() + 1 > self.slots as usize {
            return Err(InventoryOperationError::FullInventory)
        }

        if equipment.is_unique {
            if self.unique_items.contains(&equipment.item.item_id) {
                return Err(InventoryOperationError::HasUnique)
            }
            self.unique_items.insert(equipment.item.item_id.clone());
        }

        let empty_slot_idx = self.find_empty_slot()?;

        self.items[empty_slot_idx] = Some(equipment);
        Ok(())
    }

    fn remove_by_slot(&mut self, slot: i16) -> Result<(), InventoryOperationError> {
        todo!()
    }

    fn update_item(&mut self, item: Equipment, qty: i32) -> Result<(), InventoryOperationError> {
        todo!()
    }
    fn get(&self, slot: i16) -> Result<Equipment, InventoryOperationError> {
        todo!()
    }
    fn sort(&mut self) -> Result<Equipment, InventoryOperationError> {
        todo!()
    }

    fn collapse(&mut self) -> Result<Equipment, InventoryOperationError> {
        todo!()
    }

}

impl<T: Clone> Inventory<T> {
    pub fn new(id: i64, slots: i16, inv_type: InvType) -> Self {
        let mut items: Vec<Option<T>> = vec![None; slots as usize];
        Inventory {
            id,
            slots,
            inv_type,
            items,
            unique_items: HashSet::new()
        }
    }

    pub fn find_empty_slot(&self) -> Result<usize, InventoryOperationError> {
        let slot = self.items.iter().position(|item| item.is_none());
        return if let Some(slot_i) = slot {
            Ok(slot_i)
        } else {
            Err(InventoryOperationError::FullInventory)
        }
    }
}
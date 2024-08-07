#[derive(Debug)]
pub enum InventoryOperationError {
    NotFound,
    FullInventory,
    MesosOverflow,
    HasUnique,
    EmptySlot
}
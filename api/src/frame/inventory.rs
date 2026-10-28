use env::{Mark, Name, Permission, PieKind, TaskId};

/// Maximum machine leaves plus the two boot resources under one bounded lease.
pub const INVENTORY_MAX: usize = 64;
const SLOT_BASE: u64 = 0x4855_4200_0000_0000;
const ITEM_LEN: usize = 16 + 8 + 1 + 4;

/// Stable target-local slot for an item in the machine's canonical inventory order.
pub const fn inventory_slot(index: usize) -> Option<Mark> {
    if index < INVENTORY_MAX {
        Some(Mark::new(SLOT_BASE + index as u64))
    } else {
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InventoryItem {
    pub resource: Name,
    pub owner: TaskId,
    pub kind: PieKind,
    pub permission: Permission,
}

impl InventoryItem {
    const EMPTY: Self = Self {
        resource: Name::Page(env::Page::Dtb),
        owner: TaskId::new(0),
        kind: PieKind::Pole,
        permission: Permission::empty(),
    };
}

mod snapshot;
pub use self::snapshot::Inventory;

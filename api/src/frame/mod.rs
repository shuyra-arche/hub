//! 三面各一形，加一张失败域与状态码的双射表。
//! **一原语一面**：三条问各有自己的门（`bond` / `list` 挂在 `/svc/hub` 下，`claim` 挂在**每台
//! 设备那一格**上），故**没有"一答多形"的 `Union`**——每一面的答各是一个定形。这与 operator
//! 那一族正相反：那边每一位原语共用一扇门，答话才要四种形状。

pub mod bond;
pub mod claim;
pub mod dma;
mod dma_said;
pub mod inventory;
pub mod list;
mod said;
mod deed;
mod wire;
mod window;
pub mod vocab;

pub use ::wire::OK;
pub use self::bond::*;
pub use self::claim::*;
pub use self::deed::Deed;
pub use self::dma::{
    BindingId, BindingState, Close, CloseBinding, DmaClaim, DmaSaid, DmaWire,
    Enroll, Failure, Inspect, Settle, DMA_BACK, DMA_CLAIM, DMA_CLOSE, DMA_ENROLL,
    DMA_INSPECT, DMA_SETTLE, PARTY_CONTROL, PARTY_HUB, PARTY_IOMMU, PARTY_PCI, PARTY_RECOVERY,
};
pub use self::dma::Call as DmaCall;
pub use self::inventory::*;
pub use self::list::*;
pub use self::said::Said;
pub use self::vocab::*;
pub use self::window::{Window, WINDOW_LEN};
pub use self::wire::Wire;

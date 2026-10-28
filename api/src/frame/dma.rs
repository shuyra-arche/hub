//! DMA 安全绑定那一面的帧与词汇：发现结果、绑定编号、关闭理由与协调状态。
//!
//! 与普通设备账那三面（bond/list/claim）分开：普通设备的主人一没，格子当场空出来；
//! DMA 绑定在责任结清前**不**因主人死亡变为可重绑。这一面是 Hub 与
//! PCI / IOMMU / 设备恢复 / 执行实例之间的协调协议，报文里不接收可信 caller
//! （发送者由内核给出，绑定编号由 Hub 签发）。

use env::PieToken;
pub use super::dma_said::DmaSaid;

mod binding_id;
mod binding_state;
mod close;
mod close_binding;
mod dma_claim;
mod dma_wire;
mod enroll;
mod failure;
mod inspect;
mod settle;

pub use self::binding_id::BindingId;
pub use self::binding_state::BindingState;
pub use self::close::Close;
pub use self::close_binding::CloseBinding;
pub use self::dma_claim::DmaClaim;
pub use self::dma_wire::DmaWire;
pub use self::enroll::Enroll;
pub use self::failure::Failure;
pub use self::inspect::Inspect;
pub use self::settle::Settle;

/// 七个动作在报文里的码（与那一族同名）。
pub const DMA_ENROLL: u8 = 1;
pub const DMA_CLAIM: u8 = 2;
pub const DMA_CLOSE: u8 = 3;
pub const DMA_INSPECT: u8 = 4;
pub const DMA_SETTLE: u8 = 5;

/// 失败方（谁那一手没成）。
pub const PARTY_HUB: u8 = 1;
pub const PARTY_PCI: u8 = 2;
pub const PARTY_IOMMU: u8 = 3;
pub const PARTY_RECOVERY: u8 = 4;
pub const PARTY_CONTROL: u8 = 5;

/// 列已登记设备：**不提供**。设备由「哪一枚门牌」认（见 `DmaClaim`），
/// 不向普通任务开放设备清单。

/// 回信孔那一枚上的记号：**这一面自己的**（与普通设备账那三面分得开）。
pub const DMA_BACK: env::Mark = env::Mark::of("hub-dma-back");

/// 从一问里取出回信孔那一格。
fn reply_to(wire: &DmaWire) -> PieToken {
    match wire {
        DmaWire::Enroll(frame) => frame.back,
        DmaWire::Claim(frame) => frame.back,
        DmaWire::Close(frame) => frame.back,
        DmaWire::Inspect(frame) => frame.back,
        DmaWire::Settle(frame) => frame.back,
    }
}

/// 这一面的契约：一问一答、回信孔那一格随动作码走。
#[mold::contract(request = DmaWire, response = DmaSaid, mark = DMA_BACK, back = reply_to)]
pub struct Call;

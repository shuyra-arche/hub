use env::wire::Span as _;
use wire::Message;

use super::{CloseBinding, DmaClaim, Enroll, Inspect, Settle};
use super::{
    DMA_CLAIM, DMA_CLOSE, DMA_ENROLL, DMA_INSPECT, DMA_SETTLE,
};

/// 收进来的一问。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DmaWire {
    Enroll(Enroll),
    Claim(DmaClaim),
    Close(CloseBinding),
    Inspect(Inspect),
    Settle(Settle),
}

impl DmaWire {
    pub const LEN: usize = Enroll::LEN;

    pub fn take(bytes: &[u8]) -> Option<Self> {
        match *bytes.first()? {
            DMA_ENROLL => {
                let (value, end) = <Enroll as env::wire::Span>::fetch_at(bytes, 0)?;
                (end == bytes.len()).then_some(Self::Enroll(value))
            }
            DMA_CLAIM => {
                let (value, end) = <DmaClaim as env::wire::Span>::fetch_at(bytes, 0)?;
                (end == bytes.len()).then_some(Self::Claim(value))
            }
            DMA_CLOSE => {
                let (value, end) = <CloseBinding as env::wire::Span>::fetch_at(bytes, 0)?;
                (end == bytes.len()).then_some(Self::Close(value))
            }
            DMA_INSPECT => {
                let (value, end) = <Inspect as env::wire::Span>::fetch_at(bytes, 0)?;
                (end == bytes.len()).then_some(Self::Inspect(value))
            }
            DMA_SETTLE => {
                let (value, end) = <Settle as env::wire::Span>::fetch_at(bytes, 0)?;
                (end == bytes.len()).then_some(Self::Settle(value))
            }
            _ => None,
        }
    }

    pub fn store(&self, out: &mut [u8]) -> Option<usize> {
        match self {
            Self::Enroll(value) if value.op == DMA_ENROLL => value.store_at(out, 0),
            Self::Claim(value) if value.op == DMA_CLAIM => value.store_at(out, 0),
            Self::Close(value) if value.op == DMA_CLOSE => value.store_at(out, 0),
            Self::Inspect(value) if value.op == DMA_INSPECT => value.store_at(out, 0),
            Self::Settle(value) if value.op == DMA_SETTLE => value.store_at(out, 0),
            _ => None,
        }
    }
}

impl Message for DmaWire {
    type In = Self;
    type Buf = [u8; Self::LEN];
    const EMPTY: Self::Buf = [0; Self::LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        DmaWire::store(self, out)
    }

    fn fetch(bytes: &[u8]) -> Option<Self> {
        DmaWire::take(bytes)
    }
}

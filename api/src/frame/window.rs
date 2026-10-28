use alloc::string::String;
use env::wire::Span as _;
use wire::{Message, OK};

use super::vocab::LIST_MAX;

/// **一窗**：状态 ＋ 游标 ＋ 条数 ＋ 有主那一位掩码 ＋ 至多 LIST_MAX 段名字
/// **两条一次说清**：`names[..n]` 是这一窗真正答出来的那些；`held` 的第 `i` 位对应 `names[i]`
/// （`1` = 这一台此刻有主）
#[derive(env::Frame, Clone, PartialEq, Eq, Debug)]
#[frame(len = 142)]
pub struct Window {
    pub status: u8,
    pub from: u32,
    pub n: u8,
    pub held: [u8; 8],
    #[frame(count = n, fill = String::new())]
    pub names: [String; LIST_MAX],
}

/// **线上一个字节都不许动**：这一窗那一形照文件头那张表钉住
/// 这一句先红

const _: () = assert!(Window::LEN == 1 + 4 + 1 + 8 + 32 * LIST_MAX);

impl Window {
    /// 空的一窗（失败，或这一类里一台都没有）
    pub const EMPTY: Window = Window {
        status: OK,
        from: 0,
        n: 0,
        held: [0u8; 8],
        names: [const { String::new() }; LIST_MAX],
    };

    /// 第 `i` 条（`i < n` 才有）
    pub fn name(&self, i: usize) -> Option<&String> {
        (i < self.n as usize).then(|| &self.names[i])
    }

    /// 第 `i` 条此刻有没有主
    pub fn held(&self, i: usize) -> bool {
        self.held
            .get(i / 8)
            .is_some_and(|b| (b >> (i % 8)) & 1 == 1)
    }
}

/// **这一窗的缓冲那一格**：**最长那一形**（头 ＋ 满窗的名字，同一张表求和）
/// 它是 `Buf` 的容量（收的那一侧备一只满的，装得下最宽那一窗），**不是线格式的长度**
pub const WINDOW_LEN: usize = Window::LEN;

impl Message for Window {
    type In = Window;
    type Buf = [u8; WINDOW_LEN];
    const EMPTY: Self::Buf = [0u8; WINDOW_LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        self.store_at(out, 0)
    }

    /// **恰好** 头 ＋ `n` 段名字（条数越界由那一格自己拦；长度与条数对不上答 `None`——
    /// 这条判据是**本族的**，derive 不替它判）
    fn fetch(bytes: &[u8]) -> Option<Window> {
        let (window, at) = Window::fetch_at(bytes, 0)?;
        (at == bytes.len()).then_some(window)
    }
}

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: Mode,
    t_string: TString,
    t_length: TLength,
    dma_block_num: DmaBlockNum,
    start: Start,
    continue_: Continue,
    busy: Busy,
    dma_start: DmaStart,
    dma_continue: DmaContinue,
    clear_irq: ClearIrq,
    irq_ena: IrqEna,
    date: Date,
    dma_rx_reset: DmaRxReset,
    _reserved13: [u8; 0x0c],
    h_mem: [HMem; 64],
    m_mem: [MMem; 128],
}
impl RegisterBlock {
    #[doc = "0x00 - Configures SHA algorithm"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - SHA 512/t configuration register 0."]
    #[inline(always)]
    pub const fn t_string(&self) -> &TString {
        &self.t_string
    }
    #[doc = "0x08 - SHA 512/t configuration register 1."]
    #[inline(always)]
    pub const fn t_length(&self) -> &TLength {
        &self.t_length
    }
    #[doc = "0x0c - Block number register (only effective for DMA-SHA)"]
    #[inline(always)]
    pub const fn dma_block_num(&self) -> &DmaBlockNum {
        &self.dma_block_num
    }
    #[doc = "0x10 - Starts the SHA accelerator for Typical SHA operation"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x14 - Continues SHA operation (only effective in Typical SHA mode)"]
    #[inline(always)]
    pub const fn continue_(&self) -> &Continue {
        &self.continue_
    }
    #[doc = "0x18 - Represents if SHA Accelerator is busy or not"]
    #[inline(always)]
    pub const fn busy(&self) -> &Busy {
        &self.busy
    }
    #[doc = "0x1c - Starts the SHA accelerator for DMA-SHA operation"]
    #[inline(always)]
    pub const fn dma_start(&self) -> &DmaStart {
        &self.dma_start
    }
    #[doc = "0x20 - Continues SHA operation (only effective in DMA-SHA mode)"]
    #[inline(always)]
    pub const fn dma_continue(&self) -> &DmaContinue {
        &self.dma_continue
    }
    #[doc = "0x24 - DMA-SHA interrupt clear register"]
    #[inline(always)]
    pub const fn clear_irq(&self) -> &ClearIrq {
        &self.clear_irq
    }
    #[doc = "0x28 - DMA-SHA interrupt enable register"]
    #[inline(always)]
    pub const fn irq_ena(&self) -> &IrqEna {
        &self.irq_ena
    }
    #[doc = "0x2c - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x30 - DMA RX FIFO Reset Signal"]
    #[inline(always)]
    pub const fn dma_rx_reset(&self) -> &DmaRxReset {
        &self.dma_rx_reset
    }
    #[doc = "0x40..0x80 - Sha H memory which contains intermediate hash or finial hash."]
    #[inline(always)]
    pub const fn h_mem(&self, n: usize) -> &HMem {
        &self.h_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - Sha H memory which contains intermediate hash or finial hash."]
    #[inline(always)]
    pub fn h_mem_iter(&self) -> impl Iterator<Item = &HMem> {
        self.h_mem.iter()
    }
    #[doc = "0x80..0x100 - Sha M memory which contains message."]
    #[inline(always)]
    pub const fn m_mem(&self, n: usize) -> &MMem {
        &self.m_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Sha M memory which contains message."]
    #[inline(always)]
    pub fn m_mem_iter(&self) -> impl Iterator<Item = &MMem> {
        self.m_mem.iter()
    }
}
#[doc = "MODE (rw) register accessor: Configures SHA algorithm\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Configures SHA algorithm"]
pub mod mode;
#[doc = "T_STRING (rw) register accessor: SHA 512/t configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`t_string::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_string::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_string`] module"]
#[doc(alias = "T_STRING")]
pub type TString = crate::Reg<t_string::TStringSpec>;
#[doc = "SHA 512/t configuration register 0."]
pub mod t_string;
#[doc = "T_LENGTH (rw) register accessor: SHA 512/t configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`t_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_length`] module"]
#[doc(alias = "T_LENGTH")]
pub type TLength = crate::Reg<t_length::TLengthSpec>;
#[doc = "SHA 512/t configuration register 1."]
pub mod t_length;
#[doc = "DMA_BLOCK_NUM (rw) register accessor: Block number register (only effective for DMA-SHA)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_block_num`] module"]
#[doc(alias = "DMA_BLOCK_NUM")]
pub type DmaBlockNum = crate::Reg<dma_block_num::DmaBlockNumSpec>;
#[doc = "Block number register (only effective for DMA-SHA)"]
pub mod dma_block_num;
#[doc = "START (w) register accessor: Starts the SHA accelerator for Typical SHA operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Starts the SHA accelerator for Typical SHA operation"]
pub mod start;
#[doc = "CONTINUE (w) register accessor: Continues SHA operation (only effective in Typical SHA mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_`] module"]
#[doc(alias = "CONTINUE")]
pub type Continue = crate::Reg<continue_::ContinueSpec>;
#[doc = "Continues SHA operation (only effective in Typical SHA mode)"]
pub mod continue_;
#[doc = "BUSY (r) register accessor: Represents if SHA Accelerator is busy or not\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busy`] module"]
#[doc(alias = "BUSY")]
pub type Busy = crate::Reg<busy::BusySpec>;
#[doc = "Represents if SHA Accelerator is busy or not"]
pub mod busy;
#[doc = "DMA_START (w) register accessor: Starts the SHA accelerator for DMA-SHA operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_start`] module"]
#[doc(alias = "DMA_START")]
pub type DmaStart = crate::Reg<dma_start::DmaStartSpec>;
#[doc = "Starts the SHA accelerator for DMA-SHA operation"]
pub mod dma_start;
#[doc = "DMA_CONTINUE (w) register accessor: Continues SHA operation (only effective in DMA-SHA mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_continue`] module"]
#[doc(alias = "DMA_CONTINUE")]
pub type DmaContinue = crate::Reg<dma_continue::DmaContinueSpec>;
#[doc = "Continues SHA operation (only effective in DMA-SHA mode)"]
pub mod dma_continue;
#[doc = "CLEAR_IRQ (w) register accessor: DMA-SHA interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_irq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_irq`] module"]
#[doc(alias = "CLEAR_IRQ")]
pub type ClearIrq = crate::Reg<clear_irq::ClearIrqSpec>;
#[doc = "DMA-SHA interrupt clear register"]
pub mod clear_irq;
#[doc = "IRQ_ENA (rw) register accessor: DMA-SHA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_ena`] module"]
#[doc(alias = "IRQ_ENA")]
pub type IrqEna = crate::Reg<irq_ena::IrqEnaSpec>;
#[doc = "DMA-SHA interrupt enable register"]
pub mod irq_ena;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Version control register"]
pub mod date;
#[doc = "DMA_RX_RESET (w) register accessor: DMA RX FIFO Reset Signal\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_reset`] module"]
#[doc(alias = "DMA_RX_RESET")]
pub type DmaRxReset = crate::Reg<dma_rx_reset::DmaRxResetSpec>;
#[doc = "DMA RX FIFO Reset Signal"]
pub mod dma_rx_reset;
#[doc = "H_MEM (rw) register accessor: Sha H memory which contains intermediate hash or finial hash.\n\nYou can [`read`](crate::Reg::read) this register and get [`h_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`h_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_mem`] module"]
#[doc(alias = "H_MEM")]
pub type HMem = crate::Reg<h_mem::HMemSpec>;
#[doc = "Sha H memory which contains intermediate hash or finial hash."]
pub mod h_mem;
#[doc = "M_MEM (rw) register accessor: Sha M memory which contains message.\n\nYou can [`read`](crate::Reg::read) this register and get [`m_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_mem`] module"]
#[doc(alias = "M_MEM")]
pub type MMem = crate::Reg<m_mem::MMemSpec>;
#[doc = "Sha M memory which contains message."]
pub mod m_mem;

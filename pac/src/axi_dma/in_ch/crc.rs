#[repr(C)]
#[doc = "Cluster CRC, containing IN_CRC_INIT_DATA, RX_CRC_WIDTH, IN_CRC_CLEAR, IN_CRC_FINAL_RESULT, RX_CRC_EN_WR_DATA, RX_CRC_EN_ADDR, RX_CRC_DATA_EN_WR_DATA, RX_CRC_DATA_EN_ADDR"]
#[doc(alias = "CRC")]
pub struct Crc {
    in_crc_init_data: InCrcInitData,
    rx_crc_width: RxCrcWidth,
    in_crc_clear: InCrcClear,
    in_crc_final_result: InCrcFinalResult,
    rx_crc_en_wr_data: RxCrcEnWrData,
    rx_crc_en_addr: RxCrcEnAddr,
    rx_crc_data_en_wr_data: RxCrcDataEnWrData,
    rx_crc_data_en_addr: RxCrcDataEnAddr,
}
impl Crc {
    #[doc = "0x00 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn in_crc_init_data(&self) -> &InCrcInitData {
        &self.in_crc_init_data
    }
    #[doc = "0x04 - This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
    #[inline(always)]
    pub const fn rx_crc_width(&self) -> &RxCrcWidth {
        &self.rx_crc_width
    }
    #[doc = "0x08 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_clear(&self) -> &InCrcClear {
        &self.in_crc_clear
    }
    #[doc = "0x0c - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_final_result(&self) -> &InCrcFinalResult {
        &self.in_crc_final_result
    }
    #[doc = "0x10 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn rx_crc_en_wr_data(&self) -> &RxCrcEnWrData {
        &self.rx_crc_en_wr_data
    }
    #[doc = "0x14 - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn rx_crc_en_addr(&self) -> &RxCrcEnAddr {
        &self.rx_crc_en_addr
    }
    #[doc = "0x18 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data(&self) -> &RxCrcDataEnWrData {
        &self.rx_crc_data_en_wr_data
    }
    #[doc = "0x1c - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_addr(&self) -> &RxCrcDataEnAddr {
        &self.rx_crc_data_en_addr
    }
}
#[doc = "IN_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_init_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_init_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_init_data`] module"]
#[doc(alias = "IN_CRC_INIT_DATA")]
pub type InCrcInitData = crate::Reg<in_crc_init_data::InCrcInitDataSpec>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod in_crc_init_data;
#[doc = "RX_CRC_WIDTH (rw) register accessor: This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_width`] module"]
#[doc(alias = "RX_CRC_WIDTH")]
pub type RxCrcWidth = crate::Reg<rx_crc_width::RxCrcWidthSpec>;
#[doc = "This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
pub mod rx_crc_width;
#[doc = "IN_CRC_CLEAR (rw) register accessor: This register is used to clear ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_clear`] module"]
#[doc(alias = "IN_CRC_CLEAR")]
pub type InCrcClear = crate::Reg<in_crc_clear::InCrcClearSpec>;
#[doc = "This register is used to clear ch0 crc result"]
pub mod in_crc_clear;
#[doc = "IN_CRC_FINAL_RESULT (r) register accessor: This register is used to store ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_final_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_final_result`] module"]
#[doc(alias = "IN_CRC_FINAL_RESULT")]
pub type InCrcFinalResult = crate::Reg<in_crc_final_result::InCrcFinalResultSpec>;
#[doc = "This register is used to store ch0 crc result"]
pub mod in_crc_final_result;
#[doc = "RX_CRC_EN_WR_DATA (rw) register accessor: This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_wr_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_wr_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_wr_data`] module"]
#[doc(alias = "RX_CRC_EN_WR_DATA")]
pub type RxCrcEnWrData = crate::Reg<rx_crc_en_wr_data::RxCrcEnWrDataSpec>;
#[doc = "This resister is used to config ch0 crc en for every bit"]
pub mod rx_crc_en_wr_data;
#[doc = "RX_CRC_EN_ADDR (rw) register accessor: This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_addr`] module"]
#[doc(alias = "RX_CRC_EN_ADDR")]
pub type RxCrcEnAddr = crate::Reg<rx_crc_en_addr::RxCrcEnAddrSpec>;
#[doc = "This register is used to config ch0 crc en addr"]
pub mod rx_crc_en_addr;
#[doc = "RX_CRC_DATA_EN_WR_DATA (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_wr_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_wr_data`] module"]
#[doc(alias = "RX_CRC_DATA_EN_WR_DATA")]
pub type RxCrcDataEnWrData = crate::Reg<rx_crc_data_en_wr_data::RxCrcDataEnWrDataSpec>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod rx_crc_data_en_wr_data;
#[doc = "RX_CRC_DATA_EN_ADDR (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_addr`] module"]
#[doc(alias = "RX_CRC_DATA_EN_ADDR")]
pub type RxCrcDataEnAddr = crate::Reg<rx_crc_data_en_addr::RxCrcDataEnAddrSpec>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod rx_crc_data_en_addr;

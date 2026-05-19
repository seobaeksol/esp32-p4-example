#[doc = "Register `DATA_BUFFER_STATUS_LEVEL` reader"]
pub type R = crate::R<DataBufferStatusLevelSpec>;
#[doc = "Field `TX_DATA_BUF_EMPTY_CNT` reader - Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer."]
pub type TxDataBufEmptyCntR = crate::FieldReader;
#[doc = "Field `RX_DATA_BUF_CNT` reader - Receive Buffer Level value contains the number of valid data entries in the receive buffer."]
pub type RxDataBufCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer."]
    #[inline(always)]
    pub fn tx_data_buf_empty_cnt(&self) -> TxDataBufEmptyCntR {
        TxDataBufEmptyCntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receive Buffer Level value contains the number of valid data entries in the receive buffer."]
    #[inline(always)]
    pub fn rx_data_buf_cnt(&self) -> RxDataBufCntR {
        RxDataBufCntR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "DATA_BUFFER_STATUS_LEVEL reflects the status level of the Buffers in the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_buffer_status_level::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataBufferStatusLevelSpec;
impl crate::RegisterSpec for DataBufferStatusLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_buffer_status_level::R`](R) reader structure"]
impl crate::Readable for DataBufferStatusLevelSpec {}
#[doc = "`reset()` method sets DATA_BUFFER_STATUS_LEVEL to value 0x20"]
impl crate::Resettable for DataBufferStatusLevelSpec {
    const RESET_VALUE: u32 = 0x20;
}

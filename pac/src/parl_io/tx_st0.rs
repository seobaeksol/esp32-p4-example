#[doc = "Register `TX_ST0` reader"]
pub type R = crate::R<TxSt0Spec>;
#[doc = "Field `TX_CNT` reader - Indicates the cycle number of reading Tx FIFO."]
pub type TxCntR = crate::FieldReader;
#[doc = "Field `TX_FIFO_RD_BIT_CNT` reader - Indicates the current read bit number from Tx FIFO."]
pub type TxFifoRdBitCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:12 - Indicates the cycle number of reading Tx FIFO."]
    #[inline(always)]
    pub fn tx_cnt(&self) -> TxCntR {
        TxCntR::new(((self.bits >> 6) & 0x7f) as u8)
    }
    #[doc = "Bits 13:31 - Indicates the current read bit number from Tx FIFO."]
    #[inline(always)]
    pub fn tx_fifo_rd_bit_cnt(&self) -> TxFifoRdBitCntR {
        TxFifoRdBitCntR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[doc = "Parallel IO TX status register0\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_st0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxSt0Spec;
impl crate::RegisterSpec for TxSt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_st0::R`](R) reader structure"]
impl crate::Readable for TxSt0Spec {}
#[doc = "`reset()` method sets TX_ST0 to value 0"]
impl crate::Resettable for TxSt0Spec {}

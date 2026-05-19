#[doc = "Register `RX_ST1` reader"]
pub type R = crate::R<RxSt1Spec>;
#[doc = "Field `RX_FIFO_RD_BIT_CNT` reader - Indicates the current read bit number from Rx FIFO."]
pub type RxFifoRdBitCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 13:31 - Indicates the current read bit number from Rx FIFO."]
    #[inline(always)]
    pub fn rx_fifo_rd_bit_cnt(&self) -> RxFifoRdBitCntR {
        RxFifoRdBitCntR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[doc = "Parallel IO RX status register1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_st1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxSt1Spec;
impl crate::RegisterSpec for RxSt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_st1::R`](R) reader structure"]
impl crate::Readable for RxSt1Spec {}
#[doc = "`reset()` method sets RX_ST1 to value 0"]
impl crate::Resettable for RxSt1Spec {}

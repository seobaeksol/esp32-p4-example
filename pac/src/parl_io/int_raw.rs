#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Field `TX_FIFO_REMPTY` reader - The raw interrupt status of TX_FIFO_REMPTY_INT."]
pub type TxFifoRemptyR = crate::BitReader;
#[doc = "Field `RX_FIFO_WOVF` reader - The raw interrupt status of RX_FIFO_WOVF_INT."]
pub type RxFifoWovfR = crate::BitReader;
#[doc = "Field `TX_EOF` reader - The raw interrupt status of TX_EOF_INT."]
pub type TxEofR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    pub fn tx_fifo_rempty(&self) -> TxFifoRemptyR {
        TxFifoRemptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of RX_FIFO_WOVF_INT."]
    #[inline(always)]
    pub fn rx_fifo_wovf(&self) -> RxFifoWovfR {
        RxFifoWovfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of TX_EOF_INT."]
    #[inline(always)]
    pub fn tx_eof(&self) -> TxEofR {
        TxEofR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Parallel IO interrupt raw singal status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}

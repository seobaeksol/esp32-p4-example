#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `RX_DONE` reader - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` reader - The masked interrupt status bit for the i2s_tx_done_int interrupt"]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `RX_HUNG` reader - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type RxHungR = crate::BitReader;
#[doc = "Field `TX_HUNG` reader - The masked interrupt status bit for the i2s_tx_hung_int interrupt"]
pub type TxHungR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&self) -> RxHungR {
        RxHungR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn tx_hung(&self) -> TxHungR {
        TxHungR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "I2S interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}

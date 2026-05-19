#[doc = "Register `ST` reader"]
pub type R = crate::R<StSpec>;
#[doc = "Field `TX_READY` reader - Represents the status that tx is ready to transmit."]
pub type TxReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 31 - Represents the status that tx is ready to transmit."]
    #[inline(always)]
    pub fn tx_ready(&self) -> TxReadyR {
        TxReadyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Parallel IO module status register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StSpec;
impl crate::RegisterSpec for StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st::R`](R) reader structure"]
impl crate::Readable for StSpec {}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for StSpec {}

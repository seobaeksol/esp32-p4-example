#[doc = "Register `QUERY_BUSY` reader"]
pub type R = crate::R<QueryBusySpec>;
#[doc = "Field `BUSY_STATE` reader - Represents whether or not HMAC is in a busy state. Before configuring HMAC, please make sure HMAC is in an IDLE state. \\\\0: Idle \\\\1: HMAC is still working on the calculation"]
pub type BusyStateR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not HMAC is in a busy state. Before configuring HMAC, please make sure HMAC is in an IDLE state. \\\\0: Idle \\\\1: HMAC is still working on the calculation"]
    #[inline(always)]
    pub fn busy_state(&self) -> BusyStateR {
        BusyStateR::new((self.bits & 1) != 0)
    }
}
#[doc = "Busy state of HMAC module\n\nYou can [`read`](crate::Reg::read) this register and get [`query_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QueryBusySpec;
impl crate::RegisterSpec for QueryBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_busy::R`](R) reader structure"]
impl crate::Readable for QueryBusySpec {}
#[doc = "`reset()` method sets QUERY_BUSY to value 0"]
impl crate::Resettable for QueryBusySpec {}

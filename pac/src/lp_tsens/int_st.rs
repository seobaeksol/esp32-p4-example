#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `COCPU_TSENS_WAKE` reader - Tsens wakeup interrupt status."]
pub type CocpuTsensWakeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tsens wakeup interrupt status."]
    #[inline(always)]
    pub fn cocpu_tsens_wake(&self) -> CocpuTsensWakeR {
        CocpuTsensWakeR::new((self.bits & 1) != 0)
    }
}
#[doc = "Tsens interrupt status registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}

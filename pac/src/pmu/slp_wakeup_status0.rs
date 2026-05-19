#[doc = "Register `SLP_WAKEUP_STATUS0` reader"]
pub type R = crate::R<SlpWakeupStatus0Spec>;
#[doc = "Field `WAKEUP_CAUSE` reader - need_des"]
pub type WakeupCauseR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WakeupCauseR {
        WakeupCauseR::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpWakeupStatus0Spec;
impl crate::RegisterSpec for SlpWakeupStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_status0::R`](R) reader structure"]
impl crate::Readable for SlpWakeupStatus0Spec {}
#[doc = "`reset()` method sets SLP_WAKEUP_STATUS0 to value 0"]
impl crate::Resettable for SlpWakeupStatus0Spec {}

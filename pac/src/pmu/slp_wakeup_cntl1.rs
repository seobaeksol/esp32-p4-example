#[doc = "Register `SLP_WAKEUP_CNTL1` reader"]
pub type R = crate::R<SlpWakeupCntl1Spec>;
#[doc = "Register `SLP_WAKEUP_CNTL1` writer"]
pub type W = crate::W<SlpWakeupCntl1Spec>;
#[doc = "Field `SLEEP_REJECT_ENA` reader - need_des"]
pub type SleepRejectEnaR = crate::FieldReader<u32>;
#[doc = "Field `SLEEP_REJECT_ENA` writer - need_des"]
pub type SleepRejectEnaW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `SLP_REJECT_EN` reader - need_des"]
pub type SlpRejectEnR = crate::BitReader;
#[doc = "Field `SLP_REJECT_EN` writer - need_des"]
pub type SlpRejectEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn sleep_reject_ena(&self) -> SleepRejectEnaR {
        SleepRejectEnaR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn slp_reject_en(&self) -> SlpRejectEnR {
        SlpRejectEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn sleep_reject_ena(&mut self) -> SleepRejectEnaW<'_, SlpWakeupCntl1Spec> {
        SleepRejectEnaW::new(self, 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn slp_reject_en(&mut self) -> SlpRejectEnW<'_, SlpWakeupCntl1Spec> {
        SlpRejectEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpWakeupCntl1Spec;
impl crate::RegisterSpec for SlpWakeupCntl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl1::R`](R) reader structure"]
impl crate::Readable for SlpWakeupCntl1Spec {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl1::W`](W) writer structure"]
impl crate::Writable for SlpWakeupCntl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL1 to value 0"]
impl crate::Resettable for SlpWakeupCntl1Spec {}

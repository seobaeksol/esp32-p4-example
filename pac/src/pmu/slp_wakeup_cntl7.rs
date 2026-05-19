#[doc = "Register `SLP_WAKEUP_CNTL7` reader"]
pub type R = crate::R<SlpWakeupCntl7Spec>;
#[doc = "Register `SLP_WAKEUP_CNTL7` writer"]
pub type W = crate::W<SlpWakeupCntl7Spec>;
#[doc = "Field `ANA_WAIT_TARGET` reader - need_des"]
pub type AnaWaitTargetR = crate::FieldReader<u16>;
#[doc = "Field `ANA_WAIT_TARGET` writer - need_des"]
pub type AnaWaitTargetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn ana_wait_target(&self) -> AnaWaitTargetR {
        AnaWaitTargetR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn ana_wait_target(&mut self) -> AnaWaitTargetW<'_, SlpWakeupCntl7Spec> {
        AnaWaitTargetW::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpWakeupCntl7Spec;
impl crate::RegisterSpec for SlpWakeupCntl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl7::R`](R) reader structure"]
impl crate::Readable for SlpWakeupCntl7Spec {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl7::W`](W) writer structure"]
impl crate::Writable for SlpWakeupCntl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL7 to value 0x0001_0000"]
impl crate::Resettable for SlpWakeupCntl7Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}

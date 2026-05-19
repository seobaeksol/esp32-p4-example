#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TriggerSpec>;
#[doc = "Field `TRIGGER` writer - Configures whether or not to start AES operation. \\\\ 0: No effect\\\\ 1: Start\\\\"]
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to start AES operation. \\\\ 0: No effect\\\\ 1: Start\\\\"]
    #[inline(always)]
    pub fn trigger(&mut self) -> TriggerW<'_, TriggerSpec> {
        TriggerW::new(self, 0)
    }
}
#[doc = "Operation start controlling register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSpec;
impl crate::RegisterSpec for TriggerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TriggerSpec {}

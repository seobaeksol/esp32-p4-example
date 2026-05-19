#[doc = "Register `WAKEUP_SEL` reader"]
pub type R = crate::R<WakeupSelSpec>;
#[doc = "Register `WAKEUP_SEL` writer"]
pub type W = crate::W<WakeupSelSpec>;
#[doc = "Field `SAR_WAKEUP_SEL` reader - 0: ADC1. 1: ADC2."]
pub type SarWakeupSelR = crate::BitReader;
#[doc = "Field `SAR_WAKEUP_SEL` writer - 0: ADC1. 1: ADC2."]
pub type SarWakeupSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: ADC1. 1: ADC2."]
    #[inline(always)]
    pub fn sar_wakeup_sel(&self) -> SarWakeupSelR {
        SarWakeupSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: ADC1. 1: ADC2."]
    #[inline(always)]
    pub fn sar_wakeup_sel(&mut self) -> SarWakeupSelW<'_, WakeupSelSpec> {
        SarWakeupSelW::new(self, 0)
    }
}
#[doc = "Wakeup source select register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupSelSpec;
impl crate::RegisterSpec for WakeupSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_sel::R`](R) reader structure"]
impl crate::Readable for WakeupSelSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeup_sel::W`](W) writer structure"]
impl crate::Writable for WakeupSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUP_SEL to value 0"]
impl crate::Resettable for WakeupSelSpec {}

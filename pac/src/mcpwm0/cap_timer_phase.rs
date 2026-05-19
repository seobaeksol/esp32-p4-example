#[doc = "Register `CAP_TIMER_PHASE` reader"]
pub type R = crate::R<CapTimerPhaseSpec>;
#[doc = "Register `CAP_TIMER_PHASE` writer"]
pub type W = crate::W<CapTimerPhaseSpec>;
#[doc = "Field `CAP_PHASE` reader - Configures phase value for capture timer sync operation."]
pub type CapPhaseR = crate::FieldReader<u32>;
#[doc = "Field `CAP_PHASE` writer - Configures phase value for capture timer sync operation."]
pub type CapPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn cap_phase(&self) -> CapPhaseR {
        CapPhaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn cap_phase(&mut self) -> CapPhaseW<'_, CapTimerPhaseSpec> {
        CapPhaseW::new(self, 0)
    }
}
#[doc = "Capture timer sync phase register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_timer_phase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_timer_phase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapTimerPhaseSpec;
impl crate::RegisterSpec for CapTimerPhaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_timer_phase::R`](R) reader structure"]
impl crate::Readable for CapTimerPhaseSpec {}
#[doc = "`write(|w| ..)` method takes [`cap_timer_phase::W`](W) writer structure"]
impl crate::Writable for CapTimerPhaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAP_TIMER_PHASE to value 0"]
impl crate::Resettable for CapTimerPhaseSpec {}

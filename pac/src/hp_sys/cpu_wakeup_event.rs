#[doc = "Register `CPU_WAKEUP_EVENT` reader"]
pub type R = crate::R<CpuWakeupEventSpec>;
#[doc = "Register `CPU_WAKEUP_EVENT` writer"]
pub type W = crate::W<CpuWakeupEventSpec>;
#[doc = "Field `CORE0_WAKEUP_EVENT` reader - Set this bit to wake up hp core0"]
pub type Core0WakeupEventR = crate::BitReader;
#[doc = "Field `CORE0_WAKEUP_EVENT` writer - Set this bit to wake up hp core0"]
pub type Core0WakeupEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_WAKEUP_EVENT` reader - Set this bit to wake up hp core1"]
pub type Core1WakeupEventR = crate::BitReader;
#[doc = "Field `CORE1_WAKEUP_EVENT` writer - Set this bit to wake up hp core1"]
pub type Core1WakeupEventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to wake up hp core0"]
    #[inline(always)]
    pub fn core0_wakeup_event(&self) -> Core0WakeupEventR {
        Core0WakeupEventR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to wake up hp core1"]
    #[inline(always)]
    pub fn core1_wakeup_event(&self) -> Core1WakeupEventR {
        Core1WakeupEventR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to wake up hp core0"]
    #[inline(always)]
    pub fn core0_wakeup_event(&mut self) -> Core0WakeupEventW<'_, CpuWakeupEventSpec> {
        Core0WakeupEventW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to wake up hp core1"]
    #[inline(always)]
    pub fn core1_wakeup_event(&mut self) -> Core1WakeupEventW<'_, CpuWakeupEventSpec> {
        Core1WakeupEventW::new(self, 1)
    }
}
#[doc = "cpu wakeup event ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wakeup_event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeup_event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuWakeupEventSpec;
impl crate::RegisterSpec for CpuWakeupEventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_wakeup_event::R`](R) reader structure"]
impl crate::Readable for CpuWakeupEventSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_wakeup_event::W`](W) writer structure"]
impl crate::Writable for CpuWakeupEventSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_WAKEUP_EVENT to value 0"]
impl crate::Resettable for CpuWakeupEventSpec {}

#[doc = "Register `DBIAS_TIMER` reader"]
pub type R = crate::R<DbiasTimerSpec>;
#[doc = "Register `DBIAS_TIMER` writer"]
pub type W = crate::W<DbiasTimerSpec>;
#[doc = "Field `TIMER_TARGET` reader - needs field desc"]
pub type TimerTargetR = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TARGET` writer - needs field desc"]
pub type TimerTargetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER_EN` reader - needs field desc"]
pub type TimerEnR = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - needs field desc"]
pub type TimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 15:30 - needs field desc"]
    #[inline(always)]
    pub fn timer_target(&self) -> TimerTargetR {
        TimerTargetR::new(((self.bits >> 15) & 0xffff) as u16)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timer_en(&self) -> TimerEnR {
        TimerEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 15:30 - needs field desc"]
    #[inline(always)]
    pub fn timer_target(&mut self) -> TimerTargetW<'_, DbiasTimerSpec> {
        TimerTargetW::new(self, 15)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timer_en(&mut self) -> TimerEnW<'_, DbiasTimerSpec> {
        TimerEnW::new(self, 31)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasTimerSpec;
impl crate::RegisterSpec for DbiasTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_timer::R`](R) reader structure"]
impl crate::Readable for DbiasTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`dbias_timer::W`](W) writer structure"]
impl crate::Writable for DbiasTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_TIMER to value 0x7fff_8000"]
impl crate::Resettable for DbiasTimerSpec {
    const RESET_VALUE: u32 = 0x7fff_8000;
}

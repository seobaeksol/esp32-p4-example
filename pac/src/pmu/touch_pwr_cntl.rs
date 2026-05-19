#[doc = "Register `TOUCH_PWR_CNTL` reader"]
pub type R = crate::R<TouchPwrCntlSpec>;
#[doc = "Register `TOUCH_PWR_CNTL` writer"]
pub type W = crate::W<TouchPwrCntlSpec>;
#[doc = "Field `TOUCH_WAIT_CYCLES` reader - need_des"]
pub type TouchWaitCyclesR = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_WAIT_CYCLES` writer - need_des"]
pub type TouchWaitCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - need_des"]
pub type TouchSleepCyclesR = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - need_des"]
pub type TouchSleepCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_FORCE_DONE` reader - need_des"]
pub type TouchForceDoneR = crate::BitReader;
#[doc = "Field `TOUCH_FORCE_DONE` writer - need_des"]
pub type TouchForceDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SLEEP_TIMER_EN` reader - need_des"]
pub type TouchSleepTimerEnR = crate::BitReader;
#[doc = "Field `TOUCH_SLEEP_TIMER_EN` writer - need_des"]
pub type TouchSleepTimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    pub fn touch_wait_cycles(&self) -> TouchWaitCyclesR {
        TouchWaitCyclesR::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TouchSleepCyclesR {
        TouchSleepCyclesR::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn touch_force_done(&self) -> TouchForceDoneR {
        TouchForceDoneR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn touch_sleep_timer_en(&self) -> TouchSleepTimerEnR {
        TouchSleepTimerEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    pub fn touch_wait_cycles(&mut self) -> TouchWaitCyclesW<'_, TouchPwrCntlSpec> {
        TouchWaitCyclesW::new(self, 5)
    }
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&mut self) -> TouchSleepCyclesW<'_, TouchPwrCntlSpec> {
        TouchSleepCyclesW::new(self, 14)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn touch_force_done(&mut self) -> TouchForceDoneW<'_, TouchPwrCntlSpec> {
        TouchForceDoneW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn touch_sleep_timer_en(&mut self) -> TouchSleepTimerEnW<'_, TouchPwrCntlSpec> {
        TouchSleepTimerEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pwr_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pwr_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchPwrCntlSpec;
impl crate::RegisterSpec for TouchPwrCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pwr_cntl::R`](R) reader structure"]
impl crate::Readable for TouchPwrCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_pwr_cntl::W`](W) writer structure"]
impl crate::Writable for TouchPwrCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_PWR_CNTL to value 0x0019_0140"]
impl crate::Resettable for TouchPwrCntlSpec {
    const RESET_VALUE: u32 = 0x0019_0140;
}

#[doc = "Register `LP_INT_ENA` reader"]
pub type R = crate::R<LpIntEnaSpec>;
#[doc = "Register `LP_INT_ENA` writer"]
pub type W = crate::W<LpIntEnaSpec>;
#[doc = "Field `MAIN_TIMER_OVERFLOW` reader - need_des"]
pub type MainTimerOverflowR = crate::BitReader;
#[doc = "Field `MAIN_TIMER_OVERFLOW` writer - need_des"]
pub type MainTimerOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER` reader - need_des"]
pub type MainTimerR = crate::BitReader;
#[doc = "Field `MAIN_TIMER` writer - need_des"]
pub type MainTimerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow(&self) -> MainTimerOverflowR {
        MainTimerOverflowR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer(&self) -> MainTimerR {
        MainTimerR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow(&mut self) -> MainTimerOverflowW<'_, LpIntEnaSpec> {
        MainTimerOverflowW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MainTimerW<'_, LpIntEnaSpec> {
        MainTimerW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpIntEnaSpec;
impl crate::RegisterSpec for LpIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_ena::R`](R) reader structure"]
impl crate::Readable for LpIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_int_ena::W`](W) writer structure"]
impl crate::Writable for LpIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_ENA to value 0"]
impl crate::Resettable for LpIntEnaSpec {}

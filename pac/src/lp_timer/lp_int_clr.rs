#[doc = "Register `LP_INT_CLR` writer"]
pub type W = crate::W<LpIntClrSpec>;
#[doc = "Field `MAIN_TIMER_OVERFLOW` writer - need_des"]
pub type MainTimerOverflowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER` writer - need_des"]
pub type MainTimerW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow(&mut self) -> MainTimerOverflowW<'_, LpIntClrSpec> {
        MainTimerOverflowW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MainTimerW<'_, LpIntClrSpec> {
        MainTimerW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpIntClrSpec;
impl crate::RegisterSpec for LpIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_int_clr::W`](W) writer structure"]
impl crate::Writable for LpIntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xc000_0000;
}
#[doc = "`reset()` method sets LP_INT_CLR to value 0"]
impl crate::Resettable for LpIntClrSpec {}

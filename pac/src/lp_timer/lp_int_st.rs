#[doc = "Register `LP_INT_ST` reader"]
pub type R = crate::R<LpIntStSpec>;
#[doc = "Field `MAIN_TIMER_OVERFLOW` reader - need_des"]
pub type MainTimerOverflowR = crate::BitReader;
#[doc = "Field `MAIN_TIMER` reader - need_des"]
pub type MainTimerR = crate::BitReader;
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
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpIntStSpec;
impl crate::RegisterSpec for LpIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_st::R`](R) reader structure"]
impl crate::Readable for LpIntStSpec {}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LpIntStSpec {}

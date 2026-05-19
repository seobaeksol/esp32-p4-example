#[doc = "Register `LP_SLEEP_XTAL` reader"]
pub type R = crate::R<LpSleepXtalSpec>;
#[doc = "Register `LP_SLEEP_XTAL` writer"]
pub type W = crate::W<LpSleepXtalSpec>;
#[doc = "Field `LP_SLEEP_XPD_XTAL` reader - need_des"]
pub type LpSleepXpdXtalR = crate::BitReader;
#[doc = "Field `LP_SLEEP_XPD_XTAL` writer - need_des"]
pub type LpSleepXpdXtalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_xtal(&self) -> LpSleepXpdXtalR {
        LpSleepXpdXtalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_xtal(&mut self) -> LpSleepXpdXtalW<'_, LpSleepXtalSpec> {
        LpSleepXpdXtalW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_xtal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_xtal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpSleepXtalSpec;
impl crate::RegisterSpec for LpSleepXtalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_sleep_xtal::R`](R) reader structure"]
impl crate::Readable for LpSleepXtalSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_xtal::W`](W) writer structure"]
impl crate::Writable for LpSleepXtalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_SLEEP_XTAL to value 0x8000_0000"]
impl crate::Resettable for LpSleepXtalSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}

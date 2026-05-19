#[doc = "Register `LP_SLEEP_LP_CK_POWER` reader"]
pub type R = crate::R<LpSleepLpCkPowerSpec>;
#[doc = "Register `LP_SLEEP_LP_CK_POWER` writer"]
pub type W = crate::W<LpSleepLpCkPowerSpec>;
#[doc = "Field `LP_SLEEP_XPD_LPPLL` reader - need_des"]
pub type LpSleepXpdLppllR = crate::BitReader;
#[doc = "Field `LP_SLEEP_XPD_LPPLL` writer - need_des"]
pub type LpSleepXpdLppllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_XPD_XTAL32K` reader - need_des"]
pub type LpSleepXpdXtal32kR = crate::BitReader;
#[doc = "Field `LP_SLEEP_XPD_XTAL32K` writer - need_des"]
pub type LpSleepXpdXtal32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_XPD_RC32K` reader - need_des"]
pub type LpSleepXpdRc32kR = crate::BitReader;
#[doc = "Field `LP_SLEEP_XPD_RC32K` writer - need_des"]
pub type LpSleepXpdRc32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_XPD_FOSC_CLK` reader - need_des"]
pub type LpSleepXpdFoscClkR = crate::BitReader;
#[doc = "Field `LP_SLEEP_XPD_FOSC_CLK` writer - need_des"]
pub type LpSleepXpdFoscClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_PD_OSC_CLK` reader - need_des"]
pub type LpSleepPdOscClkR = crate::BitReader;
#[doc = "Field `LP_SLEEP_PD_OSC_CLK` writer - need_des"]
pub type LpSleepPdOscClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_lppll(&self) -> LpSleepXpdLppllR {
        LpSleepXpdLppllR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_xtal32k(&self) -> LpSleepXpdXtal32kR {
        LpSleepXpdXtal32kR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_rc32k(&self) -> LpSleepXpdRc32kR {
        LpSleepXpdRc32kR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_fosc_clk(&self) -> LpSleepXpdFoscClkR {
        LpSleepXpdFoscClkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_pd_osc_clk(&self) -> LpSleepPdOscClkR {
        LpSleepPdOscClkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_lppll(&mut self) -> LpSleepXpdLppllW<'_, LpSleepLpCkPowerSpec> {
        LpSleepXpdLppllW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_xtal32k(&mut self) -> LpSleepXpdXtal32kW<'_, LpSleepLpCkPowerSpec> {
        LpSleepXpdXtal32kW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_rc32k(&mut self) -> LpSleepXpdRc32kW<'_, LpSleepLpCkPowerSpec> {
        LpSleepXpdRc32kW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_fosc_clk(&mut self) -> LpSleepXpdFoscClkW<'_, LpSleepLpCkPowerSpec> {
        LpSleepXpdFoscClkW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_pd_osc_clk(&mut self) -> LpSleepPdOscClkW<'_, LpSleepLpCkPowerSpec> {
        LpSleepPdOscClkW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_ck_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_ck_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpSleepLpCkPowerSpec;
impl crate::RegisterSpec for LpSleepLpCkPowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_sleep_lp_ck_power::R`](R) reader structure"]
impl crate::Readable for LpSleepLpCkPowerSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_lp_ck_power::W`](W) writer structure"]
impl crate::Writable for LpSleepLpCkPowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_CK_POWER to value 0x4000_0000"]
impl crate::Resettable for LpSleepLpCkPowerSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}

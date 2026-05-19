#[doc = "Register `LP_AONCLKRST_LP_CLK_EN` reader"]
pub type R = crate::R<LpAonclkrstLpClkEnSpec>;
#[doc = "Register `LP_AONCLKRST_LP_CLK_EN` writer"]
pub type W = crate::W<LpAonclkrstLpClkEnSpec>;
#[doc = "Field `LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON` reader - need_des"]
pub type LpAonclkrstLpRtcXtalForceOnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON` writer - need_des"]
pub type LpAonclkrstLpRtcXtalForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_CK_EN_LP_RAM` reader - need_des"]
pub type LpAonclkrstCkEnLpRamR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_CK_EN_LP_RAM` writer - need_des"]
pub type LpAonclkrstCkEnLpRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ETM_EVENT_TICK_EN` reader - need_des"]
pub type LpAonclkrstEtmEventTickEnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ETM_EVENT_TICK_EN` writer - need_des"]
pub type LpAonclkrstEtmEventTickEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_PLL8M_CLK_FORCE_ON` reader - need_des"]
pub type LpAonclkrstPll8mClkForceOnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_PLL8M_CLK_FORCE_ON` writer - need_des"]
pub type LpAonclkrstPll8mClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_XTAL_CLK_FORCE_ON` reader - need_des"]
pub type LpAonclkrstXtalClkForceOnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_XTAL_CLK_FORCE_ON` writer - need_des"]
pub type LpAonclkrstXtalClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_FOSC_CLK_FORCE_ON` reader - need_des"]
pub type LpAonclkrstFoscClkForceOnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_FOSC_CLK_FORCE_ON` writer - need_des"]
pub type LpAonclkrstFoscClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_rtc_xtal_force_on(&self) -> LpAonclkrstLpRtcXtalForceOnR {
        LpAonclkrstLpRtcXtalForceOnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_ck_en_lp_ram(&self) -> LpAonclkrstCkEnLpRamR {
        LpAonclkrstCkEnLpRamR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_etm_event_tick_en(&self) -> LpAonclkrstEtmEventTickEnR {
        LpAonclkrstEtmEventTickEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_pll8m_clk_force_on(&self) -> LpAonclkrstPll8mClkForceOnR {
        LpAonclkrstPll8mClkForceOnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_xtal_clk_force_on(&self) -> LpAonclkrstXtalClkForceOnR {
        LpAonclkrstXtalClkForceOnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_clk_force_on(&self) -> LpAonclkrstFoscClkForceOnR {
        LpAonclkrstFoscClkForceOnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_rtc_xtal_force_on(
        &mut self,
    ) -> LpAonclkrstLpRtcXtalForceOnW<'_, LpAonclkrstLpClkEnSpec> {
        LpAonclkrstLpRtcXtalForceOnW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_ck_en_lp_ram(
        &mut self,
    ) -> LpAonclkrstCkEnLpRamW<'_, LpAonclkrstLpClkEnSpec> {
        LpAonclkrstCkEnLpRamW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_etm_event_tick_en(
        &mut self,
    ) -> LpAonclkrstEtmEventTickEnW<'_, LpAonclkrstLpClkEnSpec> {
        LpAonclkrstEtmEventTickEnW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_pll8m_clk_force_on(
        &mut self,
    ) -> LpAonclkrstPll8mClkForceOnW<'_, LpAonclkrstLpClkEnSpec> {
        LpAonclkrstPll8mClkForceOnW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_xtal_clk_force_on(
        &mut self,
    ) -> LpAonclkrstXtalClkForceOnW<'_, LpAonclkrstLpClkEnSpec> {
        LpAonclkrstXtalClkForceOnW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_clk_force_on(
        &mut self,
    ) -> LpAonclkrstFoscClkForceOnW<'_, LpAonclkrstLpClkEnSpec> {
        LpAonclkrstFoscClkForceOnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstLpClkEnSpec;
impl crate::RegisterSpec for LpAonclkrstLpClkEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_lp_clk_en::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstLpClkEnSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_lp_clk_en::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstLpClkEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_LP_CLK_EN to value 0x0800_0000"]
impl crate::Resettable for LpAonclkrstLpClkEnSpec {
    const RESET_VALUE: u32 = 0x0800_0000;
}

#[doc = "Register `LP_AONCLKRST_LP_RST_EN` reader"]
pub type R = crate::R<LpAonclkrstLpRstEnSpec>;
#[doc = "Register `LP_AONCLKRST_LP_RST_EN` writer"]
pub type W = crate::W<LpAonclkrstLpRstEnSpec>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_HUK` reader - need_des"]
pub type LpAonclkrstRstEnLpHukR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_HUK` writer - need_des"]
pub type LpAonclkrstRstEnLpHukW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_ANAPERI` reader - need_des"]
pub type LpAonclkrstRstEnLpAnaperiR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_ANAPERI` writer - need_des"]
pub type LpAonclkrstRstEnLpAnaperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_WDT` reader - need_des"]
pub type LpAonclkrstRstEnLpWdtR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_WDT` writer - need_des"]
pub type LpAonclkrstRstEnLpWdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_TIMER` reader - need_des"]
pub type LpAonclkrstRstEnLpTimerR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_TIMER` writer - need_des"]
pub type LpAonclkrstRstEnLpTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RTC` reader - need_des"]
pub type LpAonclkrstRstEnLpRtcR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RTC` writer - need_des"]
pub type LpAonclkrstRstEnLpRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_MAILBOX` reader - need_des"]
pub type LpAonclkrstRstEnLpMailboxR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_MAILBOX` writer - need_des"]
pub type LpAonclkrstRstEnLpMailboxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_AONEFUSEREG` reader - need_des"]
pub type LpAonclkrstRstEnLpAonefuseregR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_AONEFUSEREG` writer - need_des"]
pub type LpAonclkrstRstEnLpAonefuseregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RAM` reader - need_des"]
pub type LpAonclkrstRstEnLpRamR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RAM` writer - need_des"]
pub type LpAonclkrstRstEnLpRamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_huk(&self) -> LpAonclkrstRstEnLpHukR {
        LpAonclkrstRstEnLpHukR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_anaperi(&self) -> LpAonclkrstRstEnLpAnaperiR {
        LpAonclkrstRstEnLpAnaperiR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_wdt(&self) -> LpAonclkrstRstEnLpWdtR {
        LpAonclkrstRstEnLpWdtR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_timer(&self) -> LpAonclkrstRstEnLpTimerR {
        LpAonclkrstRstEnLpTimerR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_rtc(&self) -> LpAonclkrstRstEnLpRtcR {
        LpAonclkrstRstEnLpRtcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_mailbox(&self) -> LpAonclkrstRstEnLpMailboxR {
        LpAonclkrstRstEnLpMailboxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_aonefusereg(&self) -> LpAonclkrstRstEnLpAonefuseregR {
        LpAonclkrstRstEnLpAonefuseregR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_ram(&self) -> LpAonclkrstRstEnLpRamR {
        LpAonclkrstRstEnLpRamR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_huk(
        &mut self,
    ) -> LpAonclkrstRstEnLpHukW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpHukW::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_anaperi(
        &mut self,
    ) -> LpAonclkrstRstEnLpAnaperiW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpAnaperiW::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_wdt(
        &mut self,
    ) -> LpAonclkrstRstEnLpWdtW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpWdtW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_timer(
        &mut self,
    ) -> LpAonclkrstRstEnLpTimerW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpTimerW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_rtc(
        &mut self,
    ) -> LpAonclkrstRstEnLpRtcW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpRtcW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_mailbox(
        &mut self,
    ) -> LpAonclkrstRstEnLpMailboxW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpMailboxW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_aonefusereg(
        &mut self,
    ) -> LpAonclkrstRstEnLpAonefuseregW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpAonefuseregW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_ram(
        &mut self,
    ) -> LpAonclkrstRstEnLpRamW<'_, LpAonclkrstLpRstEnSpec> {
        LpAonclkrstRstEnLpRamW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_rst_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_rst_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstLpRstEnSpec;
impl crate::RegisterSpec for LpAonclkrstLpRstEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_lp_rst_en::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstLpRstEnSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_lp_rst_en::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstLpRstEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_LP_RST_EN to value 0"]
impl crate::Resettable for LpAonclkrstLpRstEnSpec {}

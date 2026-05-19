#[doc = "Register `LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL` reader"]
pub type R = crate::R<LpAonclkrstHpSdmmcEmacRstCtrlSpec>;
#[doc = "Register `LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL` writer"]
pub type W = crate::W<LpAonclkrstHpSdmmcEmacRstCtrlSpec>;
#[doc = "Field `LP_AONCLKRST_RST_EN_SDMMC` reader - hp sdmmc reset en"]
pub type LpAonclkrstRstEnSdmmcR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_SDMMC` writer - hp sdmmc reset en"]
pub type LpAonclkrstRstEnSdmmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_SDMMC` reader - hp sdmmc force norst"]
pub type LpAonclkrstForceNorstSdmmcR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_SDMMC` writer - hp sdmmc force norst"]
pub type LpAonclkrstForceNorstSdmmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_EMAC` reader - hp emac reset en"]
pub type LpAonclkrstRstEnEmacR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_EMAC` writer - hp emac reset en"]
pub type LpAonclkrstRstEnEmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_EMAC` reader - hp emac force norst"]
pub type LpAonclkrstForceNorstEmacR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_EMAC` writer - hp emac force norst"]
pub type LpAonclkrstForceNorstEmacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - hp sdmmc reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_sdmmc(&self) -> LpAonclkrstRstEnSdmmcR {
        LpAonclkrstRstEnSdmmcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hp sdmmc force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_sdmmc(&self) -> LpAonclkrstForceNorstSdmmcR {
        LpAonclkrstForceNorstSdmmcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - hp emac reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_emac(&self) -> LpAonclkrstRstEnEmacR {
        LpAonclkrstRstEnEmacR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - hp emac force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_emac(&self) -> LpAonclkrstForceNorstEmacR {
        LpAonclkrstForceNorstEmacR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - hp sdmmc reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_sdmmc(
        &mut self,
    ) -> LpAonclkrstRstEnSdmmcW<'_, LpAonclkrstHpSdmmcEmacRstCtrlSpec> {
        LpAonclkrstRstEnSdmmcW::new(self, 28)
    }
    #[doc = "Bit 29 - hp sdmmc force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_sdmmc(
        &mut self,
    ) -> LpAonclkrstForceNorstSdmmcW<'_, LpAonclkrstHpSdmmcEmacRstCtrlSpec> {
        LpAonclkrstForceNorstSdmmcW::new(self, 29)
    }
    #[doc = "Bit 30 - hp emac reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_emac(
        &mut self,
    ) -> LpAonclkrstRstEnEmacW<'_, LpAonclkrstHpSdmmcEmacRstCtrlSpec> {
        LpAonclkrstRstEnEmacW::new(self, 30)
    }
    #[doc = "Bit 31 - hp emac force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_emac(
        &mut self,
    ) -> LpAonclkrstForceNorstEmacW<'_, LpAonclkrstHpSdmmcEmacRstCtrlSpec> {
        LpAonclkrstForceNorstEmacW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstHpSdmmcEmacRstCtrlSpec;
impl crate::RegisterSpec for LpAonclkrstHpSdmmcEmacRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstHpSdmmcEmacRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstHpSdmmcEmacRstCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL to value 0"]
impl crate::Resettable for LpAonclkrstHpSdmmcEmacRstCtrlSpec {}

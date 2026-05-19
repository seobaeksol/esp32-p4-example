#[doc = "Register `POWER_PD_LPPERI_CNTL` reader"]
pub type R = crate::R<PowerPdLpperiCntlSpec>;
#[doc = "Register `POWER_PD_LPPERI_CNTL` writer"]
pub type W = crate::W<PowerPdLpperiCntlSpec>;
#[doc = "Field `FORCE_LP_PERI_RESET` reader - need_des"]
pub type ForceLpPeriResetR = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_RESET` writer - need_des"]
pub type ForceLpPeriResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_LP_PERI_ISO` reader - need_des"]
pub type ForceLpPeriIsoR = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_ISO` writer - need_des"]
pub type ForceLpPeriIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_LP_PERI_PU` reader - need_des"]
pub type ForceLpPeriPuR = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_PU` writer - need_des"]
pub type ForceLpPeriPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_LP_PERI_NO_RESET` reader - need_des"]
pub type ForceLpPeriNoResetR = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_NO_RESET` writer - need_des"]
pub type ForceLpPeriNoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_LP_PERI_NO_ISO` reader - need_des"]
pub type ForceLpPeriNoIsoR = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_NO_ISO` writer - need_des"]
pub type ForceLpPeriNoIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_LP_PERI_PD` reader - need_des"]
pub type ForceLpPeriPdR = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_PD` writer - need_des"]
pub type ForceLpPeriPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_reset(&self) -> ForceLpPeriResetR {
        ForceLpPeriResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_iso(&self) -> ForceLpPeriIsoR {
        ForceLpPeriIsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_pu(&self) -> ForceLpPeriPuR {
        ForceLpPeriPuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_no_reset(&self) -> ForceLpPeriNoResetR {
        ForceLpPeriNoResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_no_iso(&self) -> ForceLpPeriNoIsoR {
        ForceLpPeriNoIsoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_pd(&self) -> ForceLpPeriPdR {
        ForceLpPeriPdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_reset(&mut self) -> ForceLpPeriResetW<'_, PowerPdLpperiCntlSpec> {
        ForceLpPeriResetW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_iso(&mut self) -> ForceLpPeriIsoW<'_, PowerPdLpperiCntlSpec> {
        ForceLpPeriIsoW::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_pu(&mut self) -> ForceLpPeriPuW<'_, PowerPdLpperiCntlSpec> {
        ForceLpPeriPuW::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_no_reset(&mut self) -> ForceLpPeriNoResetW<'_, PowerPdLpperiCntlSpec> {
        ForceLpPeriNoResetW::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_no_iso(&mut self) -> ForceLpPeriNoIsoW<'_, PowerPdLpperiCntlSpec> {
        ForceLpPeriNoIsoW::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_pd(&mut self) -> ForceLpPeriPdW<'_, PowerPdLpperiCntlSpec> {
        ForceLpPeriPdW::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_lpperi_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_lpperi_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerPdLpperiCntlSpec;
impl crate::RegisterSpec for PowerPdLpperiCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_lpperi_cntl::R`](R) reader structure"]
impl crate::Readable for PowerPdLpperiCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_pd_lpperi_cntl::W`](W) writer structure"]
impl crate::Writable for PowerPdLpperiCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_LPPERI_CNTL to value 0x1c"]
impl crate::Resettable for PowerPdLpperiCntlSpec {
    const RESET_VALUE: u32 = 0x1c;
}

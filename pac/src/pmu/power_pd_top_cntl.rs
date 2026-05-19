#[doc = "Register `POWER_PD_TOP_CNTL` reader"]
pub type R = crate::R<PowerPdTopCntlSpec>;
#[doc = "Register `POWER_PD_TOP_CNTL` writer"]
pub type W = crate::W<PowerPdTopCntlSpec>;
#[doc = "Field `FORCE_TOP_RESET` reader - need_des"]
pub type ForceTopResetR = crate::BitReader;
#[doc = "Field `FORCE_TOP_RESET` writer - need_des"]
pub type ForceTopResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_ISO` reader - need_des"]
pub type ForceTopIsoR = crate::BitReader;
#[doc = "Field `FORCE_TOP_ISO` writer - need_des"]
pub type ForceTopIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_PU` reader - need_des"]
pub type ForceTopPuR = crate::BitReader;
#[doc = "Field `FORCE_TOP_PU` writer - need_des"]
pub type ForceTopPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_NO_RESET` reader - need_des"]
pub type ForceTopNoResetR = crate::BitReader;
#[doc = "Field `FORCE_TOP_NO_RESET` writer - need_des"]
pub type ForceTopNoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_NO_ISO` reader - need_des"]
pub type ForceTopNoIsoR = crate::BitReader;
#[doc = "Field `FORCE_TOP_NO_ISO` writer - need_des"]
pub type ForceTopNoIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_PD` reader - need_des"]
pub type ForceTopPdR = crate::BitReader;
#[doc = "Field `FORCE_TOP_PD` writer - need_des"]
pub type ForceTopPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_top_reset(&self) -> ForceTopResetR {
        ForceTopResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_top_iso(&self) -> ForceTopIsoR {
        ForceTopIsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_top_pu(&self) -> ForceTopPuR {
        ForceTopPuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_top_no_reset(&self) -> ForceTopNoResetR {
        ForceTopNoResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_top_no_iso(&self) -> ForceTopNoIsoR {
        ForceTopNoIsoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_top_pd(&self) -> ForceTopPdR {
        ForceTopPdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_top_reset(&mut self) -> ForceTopResetW<'_, PowerPdTopCntlSpec> {
        ForceTopResetW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_top_iso(&mut self) -> ForceTopIsoW<'_, PowerPdTopCntlSpec> {
        ForceTopIsoW::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_top_pu(&mut self) -> ForceTopPuW<'_, PowerPdTopCntlSpec> {
        ForceTopPuW::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_top_no_reset(&mut self) -> ForceTopNoResetW<'_, PowerPdTopCntlSpec> {
        ForceTopNoResetW::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_top_no_iso(&mut self) -> ForceTopNoIsoW<'_, PowerPdTopCntlSpec> {
        ForceTopNoIsoW::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_top_pd(&mut self) -> ForceTopPdW<'_, PowerPdTopCntlSpec> {
        ForceTopPdW::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_top_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_top_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerPdTopCntlSpec;
impl crate::RegisterSpec for PowerPdTopCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_top_cntl::R`](R) reader structure"]
impl crate::Readable for PowerPdTopCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_pd_top_cntl::W`](W) writer structure"]
impl crate::Writable for PowerPdTopCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_TOP_CNTL to value 0x1c"]
impl crate::Resettable for PowerPdTopCntlSpec {
    const RESET_VALUE: u32 = 0x1c;
}

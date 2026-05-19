#[doc = "Register `POWER_PD_CNNT_CNTL` reader"]
pub type R = crate::R<PowerPdCnntCntlSpec>;
#[doc = "Register `POWER_PD_CNNT_CNTL` writer"]
pub type W = crate::W<PowerPdCnntCntlSpec>;
#[doc = "Field `FORCE_CNNT_RESET` reader - need_des"]
pub type ForceCnntResetR = crate::BitReader;
#[doc = "Field `FORCE_CNNT_RESET` writer - need_des"]
pub type ForceCnntResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CNNT_ISO` reader - need_des"]
pub type ForceCnntIsoR = crate::BitReader;
#[doc = "Field `FORCE_CNNT_ISO` writer - need_des"]
pub type ForceCnntIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CNNT_PU` reader - need_des"]
pub type ForceCnntPuR = crate::BitReader;
#[doc = "Field `FORCE_CNNT_PU` writer - need_des"]
pub type ForceCnntPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CNNT_NO_RESET` reader - need_des"]
pub type ForceCnntNoResetR = crate::BitReader;
#[doc = "Field `FORCE_CNNT_NO_RESET` writer - need_des"]
pub type ForceCnntNoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CNNT_NO_ISO` reader - need_des"]
pub type ForceCnntNoIsoR = crate::BitReader;
#[doc = "Field `FORCE_CNNT_NO_ISO` writer - need_des"]
pub type ForceCnntNoIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CNNT_PD` reader - need_des"]
pub type ForceCnntPdR = crate::BitReader;
#[doc = "Field `FORCE_CNNT_PD` writer - need_des"]
pub type ForceCnntPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_reset(&self) -> ForceCnntResetR {
        ForceCnntResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_iso(&self) -> ForceCnntIsoR {
        ForceCnntIsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_pu(&self) -> ForceCnntPuR {
        ForceCnntPuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_no_reset(&self) -> ForceCnntNoResetR {
        ForceCnntNoResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_no_iso(&self) -> ForceCnntNoIsoR {
        ForceCnntNoIsoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_pd(&self) -> ForceCnntPdR {
        ForceCnntPdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_reset(&mut self) -> ForceCnntResetW<'_, PowerPdCnntCntlSpec> {
        ForceCnntResetW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_iso(&mut self) -> ForceCnntIsoW<'_, PowerPdCnntCntlSpec> {
        ForceCnntIsoW::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_pu(&mut self) -> ForceCnntPuW<'_, PowerPdCnntCntlSpec> {
        ForceCnntPuW::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_no_reset(&mut self) -> ForceCnntNoResetW<'_, PowerPdCnntCntlSpec> {
        ForceCnntNoResetW::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_no_iso(&mut self) -> ForceCnntNoIsoW<'_, PowerPdCnntCntlSpec> {
        ForceCnntNoIsoW::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_cnnt_pd(&mut self) -> ForceCnntPdW<'_, PowerPdCnntCntlSpec> {
        ForceCnntPdW::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_cnnt_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_cnnt_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerPdCnntCntlSpec;
impl crate::RegisterSpec for PowerPdCnntCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_cnnt_cntl::R`](R) reader structure"]
impl crate::Readable for PowerPdCnntCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_pd_cnnt_cntl::W`](W) writer structure"]
impl crate::Writable for PowerPdCnntCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_CNNT_CNTL to value 0x1c"]
impl crate::Resettable for PowerPdCnntCntlSpec {
    const RESET_VALUE: u32 = 0x1c;
}

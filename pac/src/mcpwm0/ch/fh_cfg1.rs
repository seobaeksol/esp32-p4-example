#[doc = "Register `FH_CFG1` reader"]
pub type R = crate::R<FhCfg1Spec>;
#[doc = "Register `FH_CFG1` writer"]
pub type W = crate::W<FhCfg1Spec>;
#[doc = "Field `CLR_OST` reader - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
pub type ClrOstR = crate::BitReader;
#[doc = "Field `CLR_OST` writer - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
pub type ClrOstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBCPULSE` reader - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
pub type CbcpulseR = crate::FieldReader;
#[doc = "Field `CBCPULSE` writer - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
pub type CbcpulseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORCE_CBC` reader - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
pub type ForceCbcR = crate::BitReader;
#[doc = "Field `FORCE_CBC` writer - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
pub type ForceCbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_OST` reader - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
pub type ForceOstR = crate::BitReader;
#[doc = "Field `FORCE_OST` writer - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
pub type ForceOstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
    #[inline(always)]
    pub fn clr_ost(&self) -> ClrOstR {
        ClrOstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
    #[inline(always)]
    pub fn cbcpulse(&self) -> CbcpulseR {
        CbcpulseR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn force_cbc(&self) -> ForceCbcR {
        ForceCbcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
    #[inline(always)]
    pub fn force_ost(&self) -> ForceOstR {
        ForceOstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
    #[inline(always)]
    pub fn clr_ost(&mut self) -> ClrOstW<'_, FhCfg1Spec> {
        ClrOstW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
    #[inline(always)]
    pub fn cbcpulse(&mut self) -> CbcpulseW<'_, FhCfg1Spec> {
        CbcpulseW::new(self, 1)
    }
    #[doc = "Bit 3 - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn force_cbc(&mut self) -> ForceCbcW<'_, FhCfg1Spec> {
        ForceCbcW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
    #[inline(always)]
    pub fn force_ost(&mut self) -> ForceOstW<'_, FhCfg1Spec> {
        ForceOstW::new(self, 4)
    }
}
#[doc = "Software triggers for fault handler actions configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fh_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fh_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FhCfg1Spec;
impl crate::RegisterSpec for FhCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh_cfg1::R`](R) reader structure"]
impl crate::Readable for FhCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`fh_cfg1::W`](W) writer structure"]
impl crate::Writable for FhCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FH_CFG1 to value 0"]
impl crate::Resettable for FhCfg1Spec {}

#[doc = "Register `POWER_PD_LPPERI_MASK` reader"]
pub type R = crate::R<PowerPdLpperiMaskSpec>;
#[doc = "Register `POWER_PD_LPPERI_MASK` writer"]
pub type W = crate::W<PowerPdLpperiMaskSpec>;
#[doc = "Field `XPD_LP_PERI_MASK` reader - need_des"]
pub type XpdLpPeriMaskR = crate::FieldReader;
#[doc = "Field `XPD_LP_PERI_MASK` writer - need_des"]
pub type XpdLpPeriMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PD_LP_PERI_MASK` reader - need_des"]
pub type PdLpPeriMaskR = crate::FieldReader;
#[doc = "Field `PD_LP_PERI_MASK` writer - need_des"]
pub type PdLpPeriMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn xpd_lp_peri_mask(&self) -> XpdLpPeriMaskR {
        XpdLpPeriMaskR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_lp_peri_mask(&self) -> PdLpPeriMaskR {
        PdLpPeriMaskR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn xpd_lp_peri_mask(&mut self) -> XpdLpPeriMaskW<'_, PowerPdLpperiMaskSpec> {
        XpdLpPeriMaskW::new(self, 0)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_lp_peri_mask(&mut self) -> PdLpPeriMaskW<'_, PowerPdLpperiMaskSpec> {
        PdLpPeriMaskW::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_lpperi_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_lpperi_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerPdLpperiMaskSpec;
impl crate::RegisterSpec for PowerPdLpperiMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_lpperi_mask::R`](R) reader structure"]
impl crate::Readable for PowerPdLpperiMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`power_pd_lpperi_mask::W`](W) writer structure"]
impl crate::Writable for PowerPdLpperiMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_LPPERI_MASK to value 0"]
impl crate::Resettable for PowerPdLpperiMaskSpec {}

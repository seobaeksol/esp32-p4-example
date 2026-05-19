#[doc = "Register `VDDBAT_CFG` reader"]
pub type R = crate::R<VddbatCfgSpec>;
#[doc = "Register `VDDBAT_CFG` writer"]
pub type W = crate::W<VddbatCfgSpec>;
#[doc = "Field `ANA_VDDBAT_MODE` reader - need_des"]
pub type AnaVddbatModeR = crate::FieldReader;
#[doc = "Field `VDDBAT_SW_UPDATE` writer - need_des"]
pub type VddbatSwUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn ana_vddbat_mode(&self) -> AnaVddbatModeR {
        AnaVddbatModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn vddbat_sw_update(&mut self) -> VddbatSwUpdateW<'_, VddbatCfgSpec> {
        VddbatSwUpdateW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vddbat_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddbat_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VddbatCfgSpec;
impl crate::RegisterSpec for VddbatCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vddbat_cfg::R`](R) reader structure"]
impl crate::Readable for VddbatCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`vddbat_cfg::W`](W) writer structure"]
impl crate::Writable for VddbatCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDDBAT_CFG to value 0"]
impl crate::Resettable for VddbatCfgSpec {}

#[doc = "Register `POWER_DCDC_SWITCH` reader"]
pub type R = crate::R<PowerDcdcSwitchSpec>;
#[doc = "Register `POWER_DCDC_SWITCH` writer"]
pub type W = crate::W<PowerDcdcSwitchSpec>;
#[doc = "Field `FORCE_DCDC_SWITCH_PU` reader - need_des"]
pub type ForceDcdcSwitchPuR = crate::BitReader;
#[doc = "Field `FORCE_DCDC_SWITCH_PU` writer - need_des"]
pub type ForceDcdcSwitchPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DCDC_SWITCH_PD` reader - need_des"]
pub type ForceDcdcSwitchPdR = crate::BitReader;
#[doc = "Field `FORCE_DCDC_SWITCH_PD` writer - need_des"]
pub type ForceDcdcSwitchPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_dcdc_switch_pu(&self) -> ForceDcdcSwitchPuR {
        ForceDcdcSwitchPuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_dcdc_switch_pd(&self) -> ForceDcdcSwitchPdR {
        ForceDcdcSwitchPdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_dcdc_switch_pu(&mut self) -> ForceDcdcSwitchPuW<'_, PowerDcdcSwitchSpec> {
        ForceDcdcSwitchPuW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_dcdc_switch_pd(&mut self) -> ForceDcdcSwitchPdW<'_, PowerDcdcSwitchSpec> {
        ForceDcdcSwitchPdW::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_dcdc_switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_dcdc_switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerDcdcSwitchSpec;
impl crate::RegisterSpec for PowerDcdcSwitchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_dcdc_switch::R`](R) reader structure"]
impl crate::Readable for PowerDcdcSwitchSpec {}
#[doc = "`write(|w| ..)` method takes [`power_dcdc_switch::W`](W) writer structure"]
impl crate::Writable for PowerDcdcSwitchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_DCDC_SWITCH to value 0x01"]
impl crate::Resettable for PowerDcdcSwitchSpec {
    const RESET_VALUE: u32 = 0x01;
}

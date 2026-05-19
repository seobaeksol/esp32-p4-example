#[doc = "Register `HP_ACTIVE_HP_REGULATOR1` reader"]
pub type R = crate::R<HpActiveHpRegulator1Spec>;
#[doc = "Register `HP_ACTIVE_HP_REGULATOR1` writer"]
pub type W = crate::W<HpActiveHpRegulator1Spec>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_DRV_B` reader - need_des"]
pub type HpActiveHpRegulatorDrvBR = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_DRV_B` writer - need_des"]
pub type HpActiveHpRegulatorDrvBW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_drv_b(&self) -> HpActiveHpRegulatorDrvBR {
        HpActiveHpRegulatorDrvBR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_drv_b(
        &mut self,
    ) -> HpActiveHpRegulatorDrvBW<'_, HpActiveHpRegulator1Spec> {
        HpActiveHpRegulatorDrvBW::new(self, 26)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_regulator1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_regulator1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpActiveHpRegulator1Spec;
impl crate::RegisterSpec for HpActiveHpRegulator1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_hp_regulator1::R`](R) reader structure"]
impl crate::Readable for HpActiveHpRegulator1Spec {}
#[doc = "`write(|w| ..)` method takes [`hp_active_hp_regulator1::W`](W) writer structure"]
impl crate::Writable for HpActiveHpRegulator1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_REGULATOR1 to value 0"]
impl crate::Resettable for HpActiveHpRegulator1Spec {}

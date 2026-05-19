#[doc = "Register `HP_SLEEP_HP_REGULATOR1` reader"]
pub type R = crate::R<HpSleepHpRegulator1Spec>;
#[doc = "Register `HP_SLEEP_HP_REGULATOR1` writer"]
pub type W = crate::W<HpSleepHpRegulator1Spec>;
#[doc = "Field `HP_SLEEP_HP_REGULATOR_DRV_B` reader - need_des"]
pub type HpSleepHpRegulatorDrvBR = crate::FieldReader;
#[doc = "Field `HP_SLEEP_HP_REGULATOR_DRV_B` writer - need_des"]
pub type HpSleepHpRegulatorDrvBW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_hp_regulator_drv_b(&self) -> HpSleepHpRegulatorDrvBR {
        HpSleepHpRegulatorDrvBR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_hp_regulator_drv_b(
        &mut self,
    ) -> HpSleepHpRegulatorDrvBW<'_, HpSleepHpRegulator1Spec> {
        HpSleepHpRegulatorDrvBW::new(self, 26)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_regulator1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpSleepHpRegulator1Spec;
impl crate::RegisterSpec for HpSleepHpRegulator1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_hp_regulator1::R`](R) reader structure"]
impl crate::Readable for HpSleepHpRegulator1Spec {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_hp_regulator1::W`](W) writer structure"]
impl crate::Writable for HpSleepHpRegulator1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_HP_REGULATOR1 to value 0"]
impl crate::Resettable for HpSleepHpRegulator1Spec {}

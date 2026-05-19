#[doc = "Register `HP_MODEM_HP_REGULATOR1` writer"]
pub type W = crate::W<HpModemHpRegulator1Spec>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_DRV_B` writer - need_des"]
pub type HpModemHpRegulatorDrvBW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 8:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_drv_b(
        &mut self,
    ) -> HpModemHpRegulatorDrvBW<'_, HpModemHpRegulator1Spec> {
        HpModemHpRegulatorDrvBW::new(self, 8)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_regulator1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpModemHpRegulator1Spec;
impl crate::RegisterSpec for HpModemHpRegulator1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_hp_regulator1::W`](W) writer structure"]
impl crate::Writable for HpModemHpRegulator1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_HP_REGULATOR1 to value 0"]
impl crate::Resettable for HpModemHpRegulator1Spec {}

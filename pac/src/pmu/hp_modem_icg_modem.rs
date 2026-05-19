#[doc = "Register `HP_MODEM_ICG_MODEM` writer"]
pub type W = crate::W<HpModemIcgModemSpec>;
#[doc = "Field `HP_MODEM_DIG_ICG_MODEM_CODE` writer - need_des"]
pub type HpModemDigIcgModemCodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_icg_modem_code(
        &mut self,
    ) -> HpModemDigIcgModemCodeW<'_, HpModemIcgModemSpec> {
        HpModemDigIcgModemCodeW::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_modem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpModemIcgModemSpec;
impl crate::RegisterSpec for HpModemIcgModemSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_icg_modem::W`](W) writer structure"]
impl crate::Writable for HpModemIcgModemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_ICG_MODEM to value 0"]
impl crate::Resettable for HpModemIcgModemSpec {}

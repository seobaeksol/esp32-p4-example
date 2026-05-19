#[doc = "Register `HP_MODEM_XTAL` writer"]
pub type W = crate::W<HpModemXtalSpec>;
#[doc = "Field `HP_MODEM_XPD_XTAL` writer - need_des"]
pub type HpModemXpdXtalW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_xtal(&mut self) -> HpModemXpdXtalW<'_, HpModemXtalSpec> {
        HpModemXpdXtalW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_xtal::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpModemXtalSpec;
impl crate::RegisterSpec for HpModemXtalSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_xtal::W`](W) writer structure"]
impl crate::Writable for HpModemXtalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_XTAL to value 0"]
impl crate::Resettable for HpModemXtalSpec {}

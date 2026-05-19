#[doc = "Register `IMM_MODEM_ICG` writer"]
pub type W = crate::W<ImmModemIcgSpec>;
#[doc = "Field `UPDATE_DIG_ICG_MODEM_EN` writer - need_des"]
pub type UpdateDigIcgModemEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn update_dig_icg_modem_en(&mut self) -> UpdateDigIcgModemEnW<'_, ImmModemIcgSpec> {
        UpdateDigIcgModemEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_modem_icg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImmModemIcgSpec;
impl crate::RegisterSpec for ImmModemIcgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_modem_icg::W`](W) writer structure"]
impl crate::Writable for ImmModemIcgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_MODEM_ICG to value 0"]
impl crate::Resettable for ImmModemIcgSpec {}

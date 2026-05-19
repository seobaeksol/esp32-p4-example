#[doc = "Register `HP_MODEM_BACKUP_CLK` writer"]
pub type W = crate::W<HpModemBackupClkSpec>;
#[doc = "Field `HP_MODEM_BACKUP_ICG_FUNC_EN` writer - need_des"]
pub type HpModemBackupIcgFuncEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_backup_icg_func_en(
        &mut self,
    ) -> HpModemBackupIcgFuncEnW<'_, HpModemBackupClkSpec> {
        HpModemBackupIcgFuncEnW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_backup_clk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpModemBackupClkSpec;
impl crate::RegisterSpec for HpModemBackupClkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_backup_clk::W`](W) writer structure"]
impl crate::Writable for HpModemBackupClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_BACKUP_CLK to value 0"]
impl crate::Resettable for HpModemBackupClkSpec {}

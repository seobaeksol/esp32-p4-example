#[doc = "Register `HP_MODEM_BACKUP` writer"]
pub type W = crate::W<HpModemBackupSpec>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HpSleep2modemBackupModemClkCodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MODEM_RETENTION_MODE` writer - need_des"]
pub type HpModemRetentionModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP2MODEM_RETENTION_EN` writer - need_des"]
pub type HpSleep2modemRetentionEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_CLK_SEL` writer - need_des"]
pub type HpSleep2modemBackupClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODE` writer - need_des"]
pub type HpSleep2modemBackupModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_EN` writer - need_des"]
pub type HpSleep2modemBackupEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_modem_clk_code(
        &mut self,
    ) -> HpSleep2modemBackupModemClkCodeW<'_, HpModemBackupSpec> {
        HpSleep2modemBackupModemClkCodeW::new(self, 4)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn hp_modem_retention_mode(&mut self) -> HpModemRetentionModeW<'_, HpModemBackupSpec> {
        HpModemRetentionModeW::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_retention_en(
        &mut self,
    ) -> HpSleep2modemRetentionEnW<'_, HpModemBackupSpec> {
        HpSleep2modemRetentionEnW::new(self, 11)
    }
    #[doc = "Bits 14:15 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_clk_sel(
        &mut self,
    ) -> HpSleep2modemBackupClkSelW<'_, HpModemBackupSpec> {
        HpSleep2modemBackupClkSelW::new(self, 14)
    }
    #[doc = "Bits 20:22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_mode(
        &mut self,
    ) -> HpSleep2modemBackupModeW<'_, HpModemBackupSpec> {
        HpSleep2modemBackupModeW::new(self, 20)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_en(&mut self) -> HpSleep2modemBackupEnW<'_, HpModemBackupSpec> {
        HpSleep2modemBackupEnW::new(self, 29)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_backup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpModemBackupSpec;
impl crate::RegisterSpec for HpModemBackupSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_backup::W`](W) writer structure"]
impl crate::Writable for HpModemBackupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_BACKUP to value 0"]
impl crate::Resettable for HpModemBackupSpec {}

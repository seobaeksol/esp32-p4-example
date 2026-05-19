#[doc = "Register `HP_ACTIVE_BACKUP_CLK` reader"]
pub type R = crate::R<HpActiveBackupClkSpec>;
#[doc = "Register `HP_ACTIVE_BACKUP_CLK` writer"]
pub type W = crate::W<HpActiveBackupClkSpec>;
#[doc = "Field `HP_ACTIVE_BACKUP_ICG_FUNC_EN` reader - need_des"]
pub type HpActiveBackupIcgFuncEnR = crate::FieldReader<u32>;
#[doc = "Field `HP_ACTIVE_BACKUP_ICG_FUNC_EN` writer - need_des"]
pub type HpActiveBackupIcgFuncEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_active_backup_icg_func_en(&self) -> HpActiveBackupIcgFuncEnR {
        HpActiveBackupIcgFuncEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_active_backup_icg_func_en(
        &mut self,
    ) -> HpActiveBackupIcgFuncEnW<'_, HpActiveBackupClkSpec> {
        HpActiveBackupIcgFuncEnW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_backup_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_backup_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpActiveBackupClkSpec;
impl crate::RegisterSpec for HpActiveBackupClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_backup_clk::R`](R) reader structure"]
impl crate::Readable for HpActiveBackupClkSpec {}
#[doc = "`write(|w| ..)` method takes [`hp_active_backup_clk::W`](W) writer structure"]
impl crate::Writable for HpActiveBackupClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ACTIVE_BACKUP_CLK to value 0"]
impl crate::Resettable for HpActiveBackupClkSpec {}

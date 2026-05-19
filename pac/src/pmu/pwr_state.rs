#[doc = "Register `PWR_STATE` reader"]
pub type R = crate::R<PwrStateSpec>;
#[doc = "Field `PMU_BACKUP_ST_STATE` reader - need_des"]
pub type PmuBackupStStateR = crate::FieldReader;
#[doc = "Field `PMU_LP_PWR_ST_STATE` reader - need_des"]
pub type PmuLpPwrStStateR = crate::FieldReader;
#[doc = "Field `PMU_HP_PWR_ST_STATE` reader - need_des"]
pub type PmuHpPwrStStateR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn pmu_backup_st_state(&self) -> PmuBackupStStateR {
        PmuBackupStStateR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn pmu_lp_pwr_st_state(&self) -> PmuLpPwrStStateR {
        PmuLpPwrStStateR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    pub fn pmu_hp_pwr_st_state(&self) -> PmuHpPwrStStateR {
        PmuHpPwrStStateR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrStateSpec;
impl crate::RegisterSpec for PwrStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_state::R`](R) reader structure"]
impl crate::Readable for PwrStateSpec {}
#[doc = "`reset()` method sets PWR_STATE to value 0x0080_2000"]
impl crate::Resettable for PwrStateSpec {
    const RESET_VALUE: u32 = 0x0080_2000;
}

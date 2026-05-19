#[doc = "Register `CLK_STATE0` reader"]
pub type R = crate::R<ClkState0Spec>;
#[doc = "Field `STABLE_XPD_PLL_STATE` reader - need_des"]
pub type StableXpdPllStateR = crate::FieldReader;
#[doc = "Field `STABLE_XPD_XTAL_STATE` reader - need_des"]
pub type StableXpdXtalStateR = crate::BitReader;
#[doc = "Field `PMU_ANA_XPD_PLL_I2C_STATE` reader - need_des"]
pub type PmuAnaXpdPllI2cStateR = crate::FieldReader;
#[doc = "Field `PMU_SYS_CLK_SLP_SEL_STATE` reader - need_des"]
pub type PmuSysClkSlpSelStateR = crate::BitReader;
#[doc = "Field `PMU_SYS_CLK_SEL_STATE` reader - need_des"]
pub type PmuSysClkSelStateR = crate::FieldReader;
#[doc = "Field `PMU_SYS_CLK_NO_DIV_STATE` reader - need_des"]
pub type PmuSysClkNoDivStateR = crate::BitReader;
#[doc = "Field `PMU_ICG_SYS_CLK_EN_STATE` reader - need_des"]
pub type PmuIcgSysClkEnStateR = crate::BitReader;
#[doc = "Field `PMU_ICG_MODEM_SWITCH_STATE` reader - need_des"]
pub type PmuIcgModemSwitchStateR = crate::BitReader;
#[doc = "Field `PMU_ICG_MODEM_CODE_STATE` reader - need_des"]
pub type PmuIcgModemCodeStateR = crate::FieldReader;
#[doc = "Field `PMU_ICG_SLP_SEL_STATE` reader - need_des"]
pub type PmuIcgSlpSelStateR = crate::BitReader;
#[doc = "Field `PMU_ICG_GLOBAL_XTAL_STATE` reader - need_des"]
pub type PmuIcgGlobalXtalStateR = crate::BitReader;
#[doc = "Field `PMU_ICG_GLOBAL_PLL_STATE` reader - need_des"]
pub type PmuIcgGlobalPllStateR = crate::FieldReader;
#[doc = "Field `PMU_ANA_I2C_ISO_EN_STATE` reader - need_des"]
pub type PmuAnaI2cIsoEnStateR = crate::BitReader;
#[doc = "Field `PMU_ANA_I2C_RETENTION_STATE` reader - need_des"]
pub type PmuAnaI2cRetentionStateR = crate::BitReader;
#[doc = "Field `PMU_ANA_XPD_PLL_STATE` reader - need_des"]
pub type PmuAnaXpdPllStateR = crate::FieldReader;
#[doc = "Field `PMU_ANA_XPD_XTAL_STATE` reader - need_des"]
pub type PmuAnaXpdXtalStateR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    pub fn stable_xpd_pll_state(&self) -> StableXpdPllStateR {
        StableXpdPllStateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn stable_xpd_xtal_state(&self) -> StableXpdXtalStateR {
        StableXpdXtalStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_pll_i2c_state(&self) -> PmuAnaXpdPllI2cStateR {
        PmuAnaXpdPllI2cStateR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_slp_sel_state(&self) -> PmuSysClkSlpSelStateR {
        PmuSysClkSlpSelStateR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_sel_state(&self) -> PmuSysClkSelStateR {
        PmuSysClkSelStateR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_no_div_state(&self) -> PmuSysClkNoDivStateR {
        PmuSysClkNoDivStateR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_sys_clk_en_state(&self) -> PmuIcgSysClkEnStateR {
        PmuIcgSysClkEnStateR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_modem_switch_state(&self) -> PmuIcgModemSwitchStateR {
        PmuIcgModemSwitchStateR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_modem_code_state(&self) -> PmuIcgModemCodeStateR {
        PmuIcgModemCodeStateR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_slp_sel_state(&self) -> PmuIcgSlpSelStateR {
        PmuIcgSlpSelStateR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_global_xtal_state(&self) -> PmuIcgGlobalXtalStateR {
        PmuIcgGlobalXtalStateR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_global_pll_state(&self) -> PmuIcgGlobalPllStateR {
        PmuIcgGlobalPllStateR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_i2c_iso_en_state(&self) -> PmuAnaI2cIsoEnStateR {
        PmuAnaI2cIsoEnStateR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_i2c_retention_state(&self) -> PmuAnaI2cRetentionStateR {
        PmuAnaI2cRetentionStateR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 27:30 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_pll_state(&self) -> PmuAnaXpdPllStateR {
        PmuAnaXpdPllStateR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_xtal_state(&self) -> PmuAnaXpdXtalStateR {
        PmuAnaXpdXtalStateR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkState0Spec;
impl crate::RegisterSpec for ClkState0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_state0::R`](R) reader structure"]
impl crate::Readable for ClkState0Spec {}
#[doc = "`reset()` method sets CLK_STATE0 to value 0x8000_4000"]
impl crate::Resettable for ClkState0Spec {
    const RESET_VALUE: u32 = 0x8000_4000;
}

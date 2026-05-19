#[doc = "Register `RD_REPEAT_DATA_ERR4` reader"]
pub type R = crate::R<RdRepeatDataErr4Spec>;
#[doc = "Field `_0PXA_TIEH_SEL_0_ERR` reader - Represents the programming error of EFUSE_0PXA_TIEH_SEL_0"]
pub type _0pxaTiehSel0ErrR = crate::FieldReader;
#[doc = "Field `PVT_GLITCH_EN_ERR` reader - Represents the programming error of EFUSE_PVT_GLITCH_EN"]
pub type PvtGlitchEnErrR = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_5_H_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_5_H"]
pub type KeyPurpose5HErrR = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_H_ERR` reader - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE_H"]
pub type KmDisableDeployModeHErrR = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_ERR` reader - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE"]
pub type KmDisableDeployModeErrR = crate::FieldReader;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL_ERR` reader - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
pub type XtsDpaPseudoLevelErrR = crate::FieldReader;
#[doc = "Field `HP_PWR_SRC_SEL_ERR` reader - Represents the programming error of EFUSE_HP_PWR_SRC_SEL"]
pub type HpPwrSrcSelErrR = crate::BitReader;
#[doc = "Field `SECURE_BOOT_SHA384_EN_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_SHA384_EN"]
pub type SecureBootSha384EnErrR = crate::BitReader;
#[doc = "Field `DIS_WDT_ERR` reader - Represents the programming error of EFUSE_DIS_WDT"]
pub type DisWdtErrR = crate::BitReader;
#[doc = "Field `DIS_SWD_ERR` reader - Represents the programming error of EFUSE_DIS_SWD"]
pub type DisSwdErrR = crate::BitReader;
#[doc = "Field `PVT_GLITCH_MODE_ERR` reader - Represents the programming error of EFUSE_PVT_GLITCH_MODE"]
pub type PvtGlitchModeErrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the programming error of EFUSE_0PXA_TIEH_SEL_0"]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0_err(&self) -> _0pxaTiehSel0ErrR {
        _0pxaTiehSel0ErrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents the programming error of EFUSE_PVT_GLITCH_EN"]
    #[inline(always)]
    pub fn pvt_glitch_en_err(&self) -> PvtGlitchEnErrR {
        PvtGlitchEnErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the programming error of EFUSE_KEY_PURPOSE_5_H"]
    #[inline(always)]
    pub fn key_purpose_5_h_err(&self) -> KeyPurpose5HErrR {
        KeyPurpose5HErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE_H"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_h_err(&self) -> KmDisableDeployModeHErrR {
        KmDisableDeployModeHErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_err(&self) -> KmDisableDeployModeErrR {
        KmDisableDeployModeErrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level_err(&self) -> XtsDpaPseudoLevelErrR {
        XtsDpaPseudoLevelErrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Represents the programming error of EFUSE_HP_PWR_SRC_SEL"]
    #[inline(always)]
    pub fn hp_pwr_src_sel_err(&self) -> HpPwrSrcSelErrR {
        HpPwrSrcSelErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents the programming error of EFUSE_SECURE_BOOT_SHA384_EN"]
    #[inline(always)]
    pub fn secure_boot_sha384_en_err(&self) -> SecureBootSha384EnErrR {
        SecureBootSha384EnErrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents the programming error of EFUSE_DIS_WDT"]
    #[inline(always)]
    pub fn dis_wdt_err(&self) -> DisWdtErrR {
        DisWdtErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents the programming error of EFUSE_DIS_SWD"]
    #[inline(always)]
    pub fn dis_swd_err(&self) -> DisSwdErrR {
        DisSwdErrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Represents the programming error of EFUSE_PVT_GLITCH_MODE"]
    #[inline(always)]
    pub fn pvt_glitch_mode_err(&self) -> PvtGlitchModeErrR {
        PvtGlitchModeErrR::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdRepeatDataErr4Spec;
impl crate::RegisterSpec for RdRepeatDataErr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err4::R`](R) reader structure"]
impl crate::Readable for RdRepeatDataErr4Spec {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR4 to value 0"]
impl crate::Resettable for RdRepeatDataErr4Spec {}

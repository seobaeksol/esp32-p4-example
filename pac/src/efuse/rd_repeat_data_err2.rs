#[doc = "Register `RD_REPEAT_DATA_ERR2` reader"]
pub type R = crate::R<RdRepeatDataErr2Spec>;
#[doc = "Field `KEY_PURPOSE_2_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_2"]
pub type KeyPurpose2ErrR = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_3"]
pub type KeyPurpose3ErrR = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_4"]
pub type KeyPurpose4ErrR = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_5"]
pub type KeyPurpose5ErrR = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL_ERR` reader - Represents the programming error of EFUSE_SEC_DPA_LEVEL"]
pub type SecDpaLevelErrR = crate::FieldReader;
#[doc = "Field `XTS_DPA_CLK_ENABLE_ERR` reader - Represents the programming error of EFUSE_XTS_DPA_CLK_ENABLE"]
pub type XtsDpaClkEnableErrR = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_EN"]
pub type SecureBootEnErrR = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE"]
pub type SecureBootAggressiveRevokeErrR = crate::BitReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE_H_ERR` reader - Represents the programming error of EFUSE_KM_DEPLOY_ONLY_ONCE_H"]
pub type KmDeployOnlyOnceHErrR = crate::BitReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY_H_ERR` reader - Represents the programming error of EFUSE_FORCE_USE_KEY_MANAGER_KEY_H"]
pub type ForceUseKeyManagerKeyHErrR = crate::BitReader;
#[doc = "Field `FLASH_ECC_EN_ERR` reader - Represents the programming error of EFUSE_FLASH_ECC_EN"]
pub type FlashEccEnErrR = crate::BitReader;
#[doc = "Field `DIS_USB_OTG_DOWNLOAD_MODE_ERR` reader - Represents the programming error of EFUSE_DIS_USB_OTG_DOWNLOAD_MODE"]
pub type DisUsbOtgDownloadModeErrR = crate::BitReader;
#[doc = "Field `FLASH_TPUW_ERR` reader - Represents the programming error of EFUSE_FLASH_TPUW"]
pub type FlashTpuwErrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents the programming error of EFUSE_KEY_PURPOSE_2"]
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> KeyPurpose2ErrR {
        KeyPurpose2ErrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Represents the programming error of EFUSE_KEY_PURPOSE_3"]
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> KeyPurpose3ErrR {
        KeyPurpose3ErrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Represents the programming error of EFUSE_KEY_PURPOSE_4"]
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> KeyPurpose4ErrR {
        KeyPurpose4ErrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Represents the programming error of EFUSE_KEY_PURPOSE_5"]
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> KeyPurpose5ErrR {
        KeyPurpose5ErrR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents the programming error of EFUSE_SEC_DPA_LEVEL"]
    #[inline(always)]
    pub fn sec_dpa_level_err(&self) -> SecDpaLevelErrR {
        SecDpaLevelErrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Represents the programming error of EFUSE_XTS_DPA_CLK_ENABLE"]
    #[inline(always)]
    pub fn xts_dpa_clk_enable_err(&self) -> XtsDpaClkEnableErrR {
        XtsDpaClkEnableErrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents the programming error of EFUSE_SECURE_BOOT_EN"]
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SecureBootEnErrR {
        SecureBootEnErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents the programming error of EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(&self) -> SecureBootAggressiveRevokeErrR {
        SecureBootAggressiveRevokeErrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents the programming error of EFUSE_KM_DEPLOY_ONLY_ONCE_H"]
    #[inline(always)]
    pub fn km_deploy_only_once_h_err(&self) -> KmDeployOnlyOnceHErrR {
        KmDeployOnlyOnceHErrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents the programming error of EFUSE_FORCE_USE_KEY_MANAGER_KEY_H"]
    #[inline(always)]
    pub fn force_use_key_manager_key_h_err(&self) -> ForceUseKeyManagerKeyHErrR {
        ForceUseKeyManagerKeyHErrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents the programming error of EFUSE_FLASH_ECC_EN"]
    #[inline(always)]
    pub fn flash_ecc_en_err(&self) -> FlashEccEnErrR {
        FlashEccEnErrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents the programming error of EFUSE_DIS_USB_OTG_DOWNLOAD_MODE"]
    #[inline(always)]
    pub fn dis_usb_otg_download_mode_err(&self) -> DisUsbOtgDownloadModeErrR {
        DisUsbOtgDownloadModeErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Represents the programming error of EFUSE_FLASH_TPUW"]
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FlashTpuwErrR {
        FlashTpuwErrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdRepeatDataErr2Spec;
impl crate::RegisterSpec for RdRepeatDataErr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err2::R`](R) reader structure"]
impl crate::Readable for RdRepeatDataErr2Spec {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR2 to value 0"]
impl crate::Resettable for RdRepeatDataErr2Spec {}

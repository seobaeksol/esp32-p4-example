#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub type R = crate::R<RdRepeatData2Spec>;
#[doc = "Field `KEY_PURPOSE_2` reader - Purpose of Key2."]
pub type KeyPurpose2R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3` reader - Purpose of Key3."]
pub type KeyPurpose3R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4` reader - Purpose of Key4."]
pub type KeyPurpose4R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5` reader - Purpose of Key5."]
pub type KeyPurpose5R = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL` reader - Configures the clock random divide mode to determine the dpa secure level"]
pub type SecDpaLevelR = crate::FieldReader;
#[doc = "Field `XTS_DPA_CLK_ENABLE` reader - Sets this bit to enable xts clock anti-dpa attack function."]
pub type XtsDpaClkEnableR = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Set this bit to enable secure boot."]
pub type SecureBootEnR = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Set this bit to enable revoking aggressive secure boot."]
pub type SecureBootAggressiveRevokeR = crate::BitReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE_H` reader - EFUSE_KM_DEPLOY_ONLY_ONCE and EFUSE_KM_DEPLOY_ONLY_ONCE_H together form one field: {EFUSE_KM_DEPLOY_ONLY_ONCE_H, EFUSE_KM_DEPLOY_ONLY_ONCE\\[3:0\\]}. Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type KmDeployOnlyOnceHR = crate::BitReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY_H` reader - EFUSE_FORCE_USE_KEY_MANAGER_KEY and EFUSE_FORCE_USE_KEY_MANAGER_KEY_H together form one field: {EFUSE_FORCE_USE_KEY_MANAGER_KEY_H, EFUSE_FORCE_USE_KEY_MANAGER_KEY\\[3:0\\]}. Set each bit to control whether corresponding key must come from key manager. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type ForceUseKeyManagerKeyHR = crate::BitReader;
#[doc = "Field `FLASH_ECC_EN` reader - Set this bit to enable ECC for flash boot."]
pub type FlashEccEnR = crate::BitReader;
#[doc = "Field `DIS_USB_OTG_DOWNLOAD_MODE` reader - Set this bit to disable download via USB-OTG."]
pub type DisUsbOtgDownloadModeR = crate::BitReader;
#[doc = "Field `FLASH_TPUW` reader - Configures flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the configurable value. Otherwise, the waiting time is 30."]
pub type FlashTpuwR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Purpose of Key2."]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KeyPurpose2R {
        KeyPurpose2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Purpose of Key3."]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KeyPurpose3R {
        KeyPurpose3R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Purpose of Key4."]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KeyPurpose4R {
        KeyPurpose4R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Purpose of Key5."]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KeyPurpose5R {
        KeyPurpose5R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Configures the clock random divide mode to determine the dpa secure level"]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SecDpaLevelR {
        SecDpaLevelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Sets this bit to enable xts clock anti-dpa attack function."]
    #[inline(always)]
    pub fn xts_dpa_clk_enable(&self) -> XtsDpaClkEnableR {
        XtsDpaClkEnableR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable secure boot."]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SecureBootEnR {
        SecureBootEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable revoking aggressive secure boot."]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SecureBootAggressiveRevokeR {
        SecureBootAggressiveRevokeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EFUSE_KM_DEPLOY_ONLY_ONCE and EFUSE_KM_DEPLOY_ONLY_ONCE_H together form one field: {EFUSE_KM_DEPLOY_ONLY_ONCE_H, EFUSE_KM_DEPLOY_ONLY_ONCE\\[3:0\\]}. Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn km_deploy_only_once_h(&self) -> KmDeployOnlyOnceHR {
        KmDeployOnlyOnceHR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EFUSE_FORCE_USE_KEY_MANAGER_KEY and EFUSE_FORCE_USE_KEY_MANAGER_KEY_H together form one field: {EFUSE_FORCE_USE_KEY_MANAGER_KEY_H, EFUSE_FORCE_USE_KEY_MANAGER_KEY\\[3:0\\]}. Set each bit to control whether corresponding key must come from key manager. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn force_use_key_manager_key_h(&self) -> ForceUseKeyManagerKeyHR {
        ForceUseKeyManagerKeyHR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable ECC for flash boot."]
    #[inline(always)]
    pub fn flash_ecc_en(&self) -> FlashEccEnR {
        FlashEccEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to disable download via USB-OTG."]
    #[inline(always)]
    pub fn dis_usb_otg_download_mode(&self) -> DisUsbOtgDownloadModeR {
        DisUsbOtgDownloadModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Configures flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the configurable value. Otherwise, the waiting time is 30."]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FlashTpuwR {
        FlashTpuwR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdRepeatData2Spec;
impl crate::RegisterSpec for RdRepeatData2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data2::R`](R) reader structure"]
impl crate::Readable for RdRepeatData2Spec {}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0"]
impl crate::Resettable for RdRepeatData2Spec {}

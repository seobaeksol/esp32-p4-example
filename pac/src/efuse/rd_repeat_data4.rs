#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub type R = crate::R<RdRepeatData4Spec>;
#[doc = "Field `_0PXA_TIEH_SEL_0` reader - Output LDO VO0 tieh source select. 0: 1'b1 1: sdmmc1 2: reg 3:sdmmc0"]
pub type _0pxaTiehSel0R = crate::FieldReader;
#[doc = "Field `PVT_GLITCH_EN` reader - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
pub type PvtGlitchEnR = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_5_H` reader - Purpose of Key5. The 5-th bit."]
pub type KeyPurpose5HR = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_H` reader - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type KmDisableDeployModeHR = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE` reader - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type KmDisableDeployModeR = crate::FieldReader;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL` reader - Sets this bit to control the xts pseudo-round anti-dpa attack function. 0: controlled by register. 1-3: the higer the value is, the more pseudo-rounds are inserted to the xts-aes calculation"]
pub type XtsDpaPseudoLevelR = crate::FieldReader;
#[doc = "Field `HP_PWR_SRC_SEL` reader - HP system power source select. 0:LDO 1: DCDC"]
pub type HpPwrSrcSelR = crate::BitReader;
#[doc = "Field `SECURE_BOOT_SHA384_EN` reader - Represents whether secure boot using SHA-384 is enabled. 0: disable 1: enable"]
pub type SecureBootSha384EnR = crate::BitReader;
#[doc = "Field `DIS_WDT` reader - Set this bit to disable watch dog."]
pub type DisWdtR = crate::BitReader;
#[doc = "Field `DIS_SWD` reader - Set bit to disable super-watchdog"]
pub type DisSwdR = crate::BitReader;
#[doc = "Field `PVT_GLITCH_MODE` reader - Use to configure glitch mode"]
pub type PvtGlitchModeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Output LDO VO0 tieh source select. 0: 1'b1 1: sdmmc1 2: reg 3:sdmmc0"]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0(&self) -> _0pxaTiehSel0R {
        _0pxaTiehSel0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
    #[inline(always)]
    pub fn pvt_glitch_en(&self) -> PvtGlitchEnR {
        PvtGlitchEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Purpose of Key5. The 5-th bit."]
    #[inline(always)]
    pub fn key_purpose_5_h(&self) -> KeyPurpose5HR {
        KeyPurpose5HR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_h(&self) -> KmDisableDeployModeHR {
        KmDisableDeployModeHR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn km_disable_deploy_mode(&self) -> KmDisableDeployModeR {
        KmDisableDeployModeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Sets this bit to control the xts pseudo-round anti-dpa attack function. 0: controlled by register. 1-3: the higer the value is, the more pseudo-rounds are inserted to the xts-aes calculation"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level(&self) -> XtsDpaPseudoLevelR {
        XtsDpaPseudoLevelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - HP system power source select. 0:LDO 1: DCDC"]
    #[inline(always)]
    pub fn hp_pwr_src_sel(&self) -> HpPwrSrcSelR {
        HpPwrSrcSelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents whether secure boot using SHA-384 is enabled. 0: disable 1: enable"]
    #[inline(always)]
    pub fn secure_boot_sha384_en(&self) -> SecureBootSha384EnR {
        SecureBootSha384EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to disable watch dog."]
    #[inline(always)]
    pub fn dis_wdt(&self) -> DisWdtR {
        DisWdtR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set bit to disable super-watchdog"]
    #[inline(always)]
    pub fn dis_swd(&self) -> DisSwdR {
        DisSwdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Use to configure glitch mode"]
    #[inline(always)]
    pub fn pvt_glitch_mode(&self) -> PvtGlitchModeR {
        PvtGlitchModeR::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdRepeatData4Spec;
impl crate::RegisterSpec for RdRepeatData4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data4::R`](R) reader structure"]
impl crate::Readable for RdRepeatData4Spec {}
#[doc = "`reset()` method sets RD_REPEAT_DATA4 to value 0"]
impl crate::Resettable for RdRepeatData4Spec {}

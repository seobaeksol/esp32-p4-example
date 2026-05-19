#[doc = "Register `RD_REPEAT_DATA_ERR3` reader"]
pub type R = crate::R<RdRepeatDataErr3Spec>;
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - Represents the programming error of EFUSE_DIS_DOWNLOAD_MODE"]
pub type DisDownloadModeErrR = crate::BitReader;
#[doc = "Field `DIS_DIRECT_BOOT_ERR` reader - Represents the programming error of EFUSE_DIS_DIRECT_BOOT"]
pub type DisDirectBootErrR = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR` reader - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT"]
pub type DisUsbSerialJtagRomPrintErrR = crate::BitReader;
#[doc = "Field `LOCK_KM_KEY_ERR` reader - Represents the programming error of EFUSE_LOCK_KM_KEY"]
pub type LockKmKeyErrR = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR` reader - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE"]
pub type DisUsbSerialJtagDownloadModeErrR = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - Represents the programming error of EFUSE_ENABLE_SECURITY_DOWNLOAD"]
pub type EnableSecurityDownloadErrR = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL_ERR` reader - Represents the programming error of EFUSE_UART_PRINT_CONTROL"]
pub type UartPrintControlErrR = crate::FieldReader;
#[doc = "Field `FORCE_SEND_RESUME_ERR` reader - Represents the programming error of EFUSE_FORCE_SEND_RESUME"]
pub type ForceSendResumeErrR = crate::BitReader;
#[doc = "Field `SECURE_VERSION_ERR` reader - Represents the programming error of EFUSE_SECURE_VERSION"]
pub type SecureVersionErrR = crate::FieldReader<u16>;
#[doc = "Field `SECURE_BOOT_DISABLE_FAST_WAKE_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_DISABLE_FAST_WAKE"]
pub type SecureBootDisableFastWakeErrR = crate::BitReader;
#[doc = "Field `HYS_EN_PAD_ERR` reader - Represents the programming error of EFUSE_HYS_EN_PAD"]
pub type HysEnPadErrR = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0_H_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_0_H"]
pub type KeyPurpose0HErrR = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_1_H_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_1_H"]
pub type KeyPurpose1HErrR = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_2_H_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_2_H"]
pub type KeyPurpose2HErrR = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_3_H_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_3_H"]
pub type KeyPurpose3HErrR = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_4_H_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_4_H"]
pub type KeyPurpose4HErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents the programming error of EFUSE_DIS_DOWNLOAD_MODE"]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DisDownloadModeErrR {
        DisDownloadModeErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents the programming error of EFUSE_DIS_DIRECT_BOOT"]
    #[inline(always)]
    pub fn dis_direct_boot_err(&self) -> DisDirectBootErrR {
        DisDirectBootErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print_err(&self) -> DisUsbSerialJtagRomPrintErrR {
        DisUsbSerialJtagRomPrintErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents the programming error of EFUSE_LOCK_KM_KEY"]
    #[inline(always)]
    pub fn lock_km_key_err(&self) -> LockKmKeyErrR {
        LockKmKeyErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode_err(&self) -> DisUsbSerialJtagDownloadModeErrR {
        DisUsbSerialJtagDownloadModeErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents the programming error of EFUSE_ENABLE_SECURITY_DOWNLOAD"]
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> EnableSecurityDownloadErrR {
        EnableSecurityDownloadErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Represents the programming error of EFUSE_UART_PRINT_CONTROL"]
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UartPrintControlErrR {
        UartPrintControlErrR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Represents the programming error of EFUSE_FORCE_SEND_RESUME"]
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> ForceSendResumeErrR {
        ForceSendResumeErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:24 - Represents the programming error of EFUSE_SECURE_VERSION"]
    #[inline(always)]
    pub fn secure_version_err(&self) -> SecureVersionErrR {
        SecureVersionErrR::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - Represents the programming error of EFUSE_SECURE_BOOT_DISABLE_FAST_WAKE"]
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake_err(&self) -> SecureBootDisableFastWakeErrR {
        SecureBootDisableFastWakeErrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents the programming error of EFUSE_HYS_EN_PAD"]
    #[inline(always)]
    pub fn hys_en_pad_err(&self) -> HysEnPadErrR {
        HysEnPadErrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents the programming error of EFUSE_KEY_PURPOSE_0_H"]
    #[inline(always)]
    pub fn key_purpose_0_h_err(&self) -> KeyPurpose0HErrR {
        KeyPurpose0HErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents the programming error of EFUSE_KEY_PURPOSE_1_H"]
    #[inline(always)]
    pub fn key_purpose_1_h_err(&self) -> KeyPurpose1HErrR {
        KeyPurpose1HErrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents the programming error of EFUSE_KEY_PURPOSE_2_H"]
    #[inline(always)]
    pub fn key_purpose_2_h_err(&self) -> KeyPurpose2HErrR {
        KeyPurpose2HErrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents the programming error of EFUSE_KEY_PURPOSE_3_H"]
    #[inline(always)]
    pub fn key_purpose_3_h_err(&self) -> KeyPurpose3HErrR {
        KeyPurpose3HErrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents the programming error of EFUSE_KEY_PURPOSE_4_H"]
    #[inline(always)]
    pub fn key_purpose_4_h_err(&self) -> KeyPurpose4HErrR {
        KeyPurpose4HErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdRepeatDataErr3Spec;
impl crate::RegisterSpec for RdRepeatDataErr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err3::R`](R) reader structure"]
impl crate::Readable for RdRepeatDataErr3Spec {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR3 to value 0"]
impl crate::Resettable for RdRepeatDataErr3Spec {}

#[doc = "Register `RD_REPEAT_DATA0` reader"]
pub type R = crate::R<RdRepeatData0Spec>;
#[doc = "Field `RD_DIS` reader - Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type RdDisR = crate::FieldReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_0_1` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RecoveryBootloaderFlashSector0_1R = crate::FieldReader;
#[doc = "Field `DIS_USB_JTAG` reader - Set this bit to disable function of usb switch to jtag in module of usb device."]
pub type DisUsbJtagR = crate::BitReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_2_2` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RecoveryBootloaderFlashSector2_2R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG` reader - Set this bit to disable USB-Serial-JTAG."]
pub type DisUsbSerialJtagR = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD` reader - Set this bit to disable the function that forces chip into download mode."]
pub type DisForceDownloadR = crate::BitReader;
#[doc = "Field `SPI_DOWNLOAD_MSPI_DIS` reader - Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download."]
pub type SpiDownloadMspiDisR = crate::BitReader;
#[doc = "Field `DIS_TWAI` reader - Set this bit to disable TWAI function."]
pub type DisTwaiR = crate::BitReader;
#[doc = "Field `JTAG_SEL_ENABLE` reader - Set this bit to enable selection between usb_to_jtag and pad_to_jtag through strapping gpio25 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0."]
pub type JtagSelEnableR = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG` reader - Set odd bits to disable JTAG in the soft way. JTAG can be enabled in HMAC module."]
pub type SoftDisJtagR = crate::FieldReader;
#[doc = "Field `DIS_PAD_JTAG` reader - Set this bit to disable JTAG in the hard way. JTAG is disabled permanently."]
pub type DisPadJtagR = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - Set this bit to disable flash manual encrypt function (except in SPI boot mode)."]
pub type DisDownloadManualEncryptR = crate::BitReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_3_6` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RecoveryBootloaderFlashSector3_6R = crate::FieldReader;
#[doc = "Field `USB_PHY_SEL` reader - 0: intphy(gpio24/25) <---> usb_device\\\\ 1: intphy(26/27) <---> usb_otg11.1: intphy(gpio26/27) <---> usb_device\\\\ 1: intphy(24/25) <---> usb_otg11."]
pub type UsbPhySelR = crate::BitReader;
#[doc = "Field `HUK_GEN_STATE` reader - Set the bits to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
pub type HukGenStateR = crate::FieldReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_7_7` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RecoveryBootloaderFlashSector7_7R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RdDisR {
        RdDisR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_0_1(&self) -> RecoveryBootloaderFlashSector0_1R {
        RecoveryBootloaderFlashSector0_1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Set this bit to disable function of usb switch to jtag in module of usb device."]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DisUsbJtagR {
        DisUsbJtagR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_2_2(&self) -> RecoveryBootloaderFlashSector2_2R {
        RecoveryBootloaderFlashSector2_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to disable USB-Serial-JTAG."]
    #[inline(always)]
    pub fn dis_usb_serial_jtag(&self) -> DisUsbSerialJtagR {
        DisUsbSerialJtagR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to disable the function that forces chip into download mode."]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DisForceDownloadR {
        DisForceDownloadR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download."]
    #[inline(always)]
    pub fn spi_download_mspi_dis(&self) -> SpiDownloadMspiDisR {
        SpiDownloadMspiDisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to disable TWAI function."]
    #[inline(always)]
    pub fn dis_twai(&self) -> DisTwaiR {
        DisTwaiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable selection between usb_to_jtag and pad_to_jtag through strapping gpio25 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0."]
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> JtagSelEnableR {
        JtagSelEnableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Set odd bits to disable JTAG in the soft way. JTAG can be enabled in HMAC module."]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SoftDisJtagR {
        SoftDisJtagR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Set this bit to disable JTAG in the hard way. JTAG is disabled permanently."]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DisPadJtagR {
        DisPadJtagR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to disable flash manual encrypt function (except in SPI boot mode)."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DisDownloadManualEncryptR {
        DisDownloadManualEncryptR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:24 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_3_6(&self) -> RecoveryBootloaderFlashSector3_6R {
        RecoveryBootloaderFlashSector3_6R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - 0: intphy(gpio24/25) <---> usb_device\\\\ 1: intphy(26/27) <---> usb_otg11.1: intphy(gpio26/27) <---> usb_device\\\\ 1: intphy(24/25) <---> usb_otg11."]
    #[inline(always)]
    pub fn usb_phy_sel(&self) -> UsbPhySelR {
        UsbPhySelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Set the bits to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
    #[inline(always)]
    pub fn huk_gen_state(&self) -> HukGenStateR {
        HukGenStateR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_7_7(&self) -> RecoveryBootloaderFlashSector7_7R {
        RecoveryBootloaderFlashSector7_7R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdRepeatData0Spec;
impl crate::RegisterSpec for RdRepeatData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data0::R`](R) reader structure"]
impl crate::Readable for RdRepeatData0Spec {}
#[doc = "`reset()` method sets RD_REPEAT_DATA0 to value 0"]
impl crate::Resettable for RdRepeatData0Spec {}

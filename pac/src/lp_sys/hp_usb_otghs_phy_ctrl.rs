#[doc = "Register `HP_USB_OTGHS_PHY_CTRL` reader"]
pub type R = crate::R<HpUsbOtghsPhyCtrlSpec>;
#[doc = "Register `HP_USB_OTGHS_PHY_CTRL` writer"]
pub type W = crate::W<HpUsbOtghsPhyCtrlSpec>;
#[doc = "Field `HP_UTMIOTG_IDPULLUP` reader - Set this bit to pull up USB OTG2.0 PHY id"]
pub type HpUtmiotgIdpullupR = crate::BitReader;
#[doc = "Field `HP_UTMIOTG_IDPULLUP` writer - Set this bit to pull up USB OTG2.0 PHY id"]
pub type HpUtmiotgIdpullupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMIOTG_DPPULLDOWN` reader - Set this bit to pull down USB OTG2.0 PHY dp"]
pub type HpUtmiotgDppulldownR = crate::BitReader;
#[doc = "Field `HP_UTMIOTG_DPPULLDOWN` writer - Set this bit to pull down USB OTG2.0 PHY dp"]
pub type HpUtmiotgDppulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMIOTG_DMPULLDOWN` reader - Set this bit to pull down USB OTG2.0 PHY dm"]
pub type HpUtmiotgDmpulldownR = crate::BitReader;
#[doc = "Field `HP_UTMIOTG_DMPULLDOWN` writer - Set this bit to pull down USB OTG2.0 PHY dm"]
pub type HpUtmiotgDmpulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMISRP_CHRGVBUS` reader - Set this bit to charge USB OTG2.0 PHY vbus"]
pub type HpUtmisrpChrgvbusR = crate::BitReader;
#[doc = "Field `HP_UTMISRP_CHRGVBUS` writer - Set this bit to charge USB OTG2.0 PHY vbus"]
pub type HpUtmisrpChrgvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMISRP_DISCHRGVBUS` reader - Set this bit to discharge USB OTG2.0 PHY vbus"]
pub type HpUtmisrpDischrgvbusR = crate::BitReader;
#[doc = "Field `HP_UTMISRP_DISCHRGVBUS` writer - Set this bit to discharge USB OTG2.0 PHY vbus"]
pub type HpUtmisrpDischrgvbusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to pull up USB OTG2.0 PHY id"]
    #[inline(always)]
    pub fn hp_utmiotg_idpullup(&self) -> HpUtmiotgIdpullupR {
        HpUtmiotgIdpullupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to pull down USB OTG2.0 PHY dp"]
    #[inline(always)]
    pub fn hp_utmiotg_dppulldown(&self) -> HpUtmiotgDppulldownR {
        HpUtmiotgDppulldownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to pull down USB OTG2.0 PHY dm"]
    #[inline(always)]
    pub fn hp_utmiotg_dmpulldown(&self) -> HpUtmiotgDmpulldownR {
        HpUtmiotgDmpulldownR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to charge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_chrgvbus(&self) -> HpUtmisrpChrgvbusR {
        HpUtmisrpChrgvbusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to discharge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_dischrgvbus(&self) -> HpUtmisrpDischrgvbusR {
        HpUtmisrpDischrgvbusR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to pull up USB OTG2.0 PHY id"]
    #[inline(always)]
    pub fn hp_utmiotg_idpullup(&mut self) -> HpUtmiotgIdpullupW<'_, HpUsbOtghsPhyCtrlSpec> {
        HpUtmiotgIdpullupW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to pull down USB OTG2.0 PHY dp"]
    #[inline(always)]
    pub fn hp_utmiotg_dppulldown(&mut self) -> HpUtmiotgDppulldownW<'_, HpUsbOtghsPhyCtrlSpec> {
        HpUtmiotgDppulldownW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to pull down USB OTG2.0 PHY dm"]
    #[inline(always)]
    pub fn hp_utmiotg_dmpulldown(&mut self) -> HpUtmiotgDmpulldownW<'_, HpUsbOtghsPhyCtrlSpec> {
        HpUtmiotgDmpulldownW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to charge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_chrgvbus(&mut self) -> HpUtmisrpChrgvbusW<'_, HpUsbOtghsPhyCtrlSpec> {
        HpUtmisrpChrgvbusW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to discharge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_dischrgvbus(&mut self) -> HpUtmisrpDischrgvbusW<'_, HpUsbOtghsPhyCtrlSpec> {
        HpUtmisrpDischrgvbusW::new(self, 4)
    }
}
#[doc = "Usb otg2.0 PHY control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_usb_otghs_phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_usb_otghs_phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpUsbOtghsPhyCtrlSpec;
impl crate::RegisterSpec for HpUsbOtghsPhyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_usb_otghs_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for HpUsbOtghsPhyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hp_usb_otghs_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for HpUsbOtghsPhyCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_USB_OTGHS_PHY_CTRL to value 0x07"]
impl crate::Resettable for HpUsbOtghsPhyCtrlSpec {
    const RESET_VALUE: u32 = 0x07;
}

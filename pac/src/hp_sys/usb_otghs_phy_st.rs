#[doc = "Register `USB_OTGHS_PHY_ST` reader"]
pub type R = crate::R<UsbOtghsPhyStSpec>;
#[doc = "Field `USB_SOFT_RESET_ACTV_PDOMAIN` reader - Todo"]
pub type UsbSoftResetActvPdomainR = crate::BitReader;
#[doc = "Field `UTMISRP_SESSEND` reader - Todo"]
pub type UtmisrpSessendR = crate::BitReader;
#[doc = "Field `UTMIOTG_VBUSVALID` reader - Todo"]
pub type UtmiotgVbusvalidR = crate::BitReader;
#[doc = "Field `UTMISRP_BVALID` reader - Todo"]
pub type UtmisrpBvalidR = crate::BitReader;
#[doc = "Field `UTMISRP_SESSVALID` reader - Todo"]
pub type UtmisrpSessvalidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Todo"]
    #[inline(always)]
    pub fn usb_soft_reset_actv_pdomain(&self) -> UsbSoftResetActvPdomainR {
        UsbSoftResetActvPdomainR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Todo"]
    #[inline(always)]
    pub fn utmisrp_sessend(&self) -> UtmisrpSessendR {
        UtmisrpSessendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Todo"]
    #[inline(always)]
    pub fn utmiotg_vbusvalid(&self) -> UtmiotgVbusvalidR {
        UtmiotgVbusvalidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Todo"]
    #[inline(always)]
    pub fn utmisrp_bvalid(&self) -> UtmisrpBvalidR {
        UtmisrpBvalidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Todo"]
    #[inline(always)]
    pub fn utmisrp_sessvalid(&self) -> UtmisrpSessvalidR {
        UtmisrpSessvalidR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Usb otg2.0 PHY status register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_phy_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbOtghsPhyStSpec;
impl crate::RegisterSpec for UsbOtghsPhyStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_otghs_phy_st::R`](R) reader structure"]
impl crate::Readable for UsbOtghsPhyStSpec {}
#[doc = "`reset()` method sets USB_OTGHS_PHY_ST to value 0"]
impl crate::Resettable for UsbOtghsPhyStSpec {}

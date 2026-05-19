#[doc = "Register `BUS_RESET_ST` reader"]
pub type R = crate::R<BusResetStSpec>;
#[doc = "Field `USB_BUS_RESET_ST` reader - USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released."]
pub type UsbBusResetStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released."]
    #[inline(always)]
    pub fn usb_bus_reset_st(&self) -> UsbBusResetStR {
        UsbBusResetStR::new((self.bits & 1) != 0)
    }
}
#[doc = "USB Bus reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_reset_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusResetStSpec;
impl crate::RegisterSpec for BusResetStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_reset_st::R`](R) reader structure"]
impl crate::Readable for BusResetStSpec {}
#[doc = "`reset()` method sets BUS_RESET_ST to value 0x01"]
impl crate::Resettable for BusResetStSpec {
    const RESET_VALUE: u32 = 0x01;
}

#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Field `USB_WRAP_DATE` reader - Date register."]
pub type UsbWrapDateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Date register."]
    #[inline(always)]
    pub fn usb_wrap_date(&self) -> UsbWrapDateR {
        UsbWrapDateR::new(self.bits)
    }
}
#[doc = "Date register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`reset()` method sets DATE to value 0x2303_0504"]
impl crate::Resettable for DateSpec {
    const RESET_VALUE: u32 = 0x2303_0504;
}

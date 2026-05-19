#[doc = "Register `RD_MAC_SYS1` reader"]
pub type R = crate::R<RdMacSys1Spec>;
#[doc = "Field `MAC_1` reader - Represents MAC address. High 16-bit."]
pub type Mac1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Represents MAC address. High 16-bit."]
    #[inline(always)]
    pub fn mac_1(&self) -> Mac1R {
        Mac1R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdMacSys1Spec;
impl crate::RegisterSpec for RdMacSys1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys1::R`](R) reader structure"]
impl crate::Readable for RdMacSys1Spec {}
#[doc = "`reset()` method sets RD_MAC_SYS1 to value 0"]
impl crate::Resettable for RdMacSys1Spec {}

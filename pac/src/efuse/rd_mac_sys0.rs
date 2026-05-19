#[doc = "Register `RD_MAC_SYS0` reader"]
pub type R = crate::R<RdMacSys0Spec>;
#[doc = "Field `MAC_0` reader - Represents MAC address. Low 32-bit."]
pub type Mac0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents MAC address. Low 32-bit."]
    #[inline(always)]
    pub fn mac_0(&self) -> Mac0R {
        Mac0R::new(self.bits)
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdMacSys0Spec;
impl crate::RegisterSpec for RdMacSys0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys0::R`](R) reader structure"]
impl crate::Readable for RdMacSys0Spec {}
#[doc = "`reset()` method sets RD_MAC_SYS0 to value 0"]
impl crate::Resettable for RdMacSys0Spec {}

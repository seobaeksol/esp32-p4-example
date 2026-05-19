#[doc = "Register `DSCADDR` reader"]
pub type R = crate::R<DscaddrSpec>;
#[doc = "Field `DSCADDR` reader - Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC."]
pub type DscaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC."]
    #[inline(always)]
    pub fn dscaddr(&self) -> DscaddrR {
        DscaddrR::new(self.bits)
    }
}
#[doc = "Host descriptor address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`dscaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscaddrSpec;
impl crate::RegisterSpec for DscaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscaddr::R`](R) reader structure"]
impl crate::Readable for DscaddrSpec {}
#[doc = "`reset()` method sets DSCADDR to value 0"]
impl crate::Resettable for DscaddrSpec {}

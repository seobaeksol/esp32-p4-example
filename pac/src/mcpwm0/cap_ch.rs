#[doc = "Register `CAP_CH%s` reader"]
pub type R = crate::R<CapChSpec>;
#[doc = "Field `VALUE` reader - Represents value of last capture on CAP%s"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents value of last capture on CAP%s"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "CAP%s capture value register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapChSpec;
impl crate::RegisterSpec for CapChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch::R`](R) reader structure"]
impl crate::Readable for CapChSpec {}
#[doc = "`reset()` method sets CAP_CH%s to value 0"]
impl crate::Resettable for CapChSpec {}

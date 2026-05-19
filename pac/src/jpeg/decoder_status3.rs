#[doc = "Register `DECODER_STATUS3` reader"]
pub type R = crate::R<DecoderStatus3Spec>;
#[doc = "Field `LOOKUP_DATA` reader - Reserved"]
pub type LookupDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn lookup_data(&self) -> LookupDataR {
        LookupDataR::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder_status3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecoderStatus3Spec;
impl crate::RegisterSpec for DecoderStatus3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder_status3::R`](R) reader structure"]
impl crate::Readable for DecoderStatus3Spec {}
#[doc = "`reset()` method sets DECODER_STATUS3 to value 0"]
impl crate::Resettable for DecoderStatus3Spec {}

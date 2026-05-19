#[doc = "Register `DECODER_STATUS0` reader"]
pub type R = crate::R<DecoderStatus0Spec>;
#[doc = "Field `DECODE_BYTE_CNT` reader - Reserved"]
pub type DecodeByteCntR = crate::FieldReader<u32>;
#[doc = "Field `HEADER_DEC_ST` reader - Reserved"]
pub type HeaderDecStR = crate::FieldReader;
#[doc = "Field `DECODE_SAMPLE_SEL` reader - Reserved"]
pub type DecodeSampleSelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:25 - Reserved"]
    #[inline(always)]
    pub fn decode_byte_cnt(&self) -> DecodeByteCntR {
        DecodeByteCntR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:29 - Reserved"]
    #[inline(always)]
    pub fn header_dec_st(&self) -> HeaderDecStR {
        HeaderDecStR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn decode_sample_sel(&self) -> DecodeSampleSelR {
        DecodeSampleSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecoderStatus0Spec;
impl crate::RegisterSpec for DecoderStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder_status0::R`](R) reader structure"]
impl crate::Readable for DecoderStatus0Spec {}
#[doc = "`reset()` method sets DECODER_STATUS0 to value 0"]
impl crate::Resettable for DecoderStatus0Spec {}

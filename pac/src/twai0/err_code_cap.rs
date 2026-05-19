#[doc = "Register `ERR_CODE_CAP` reader"]
pub type R = crate::R<ErrCodeCapSpec>;
#[doc = "Field `ERR_CAPTURE_CODE_SEGMENT` reader - This register contains information about the location of errors on the bus."]
pub type ErrCaptureCodeSegmentR = crate::FieldReader;
#[doc = "Field `ERR_CAPTURE_CODE_DIRECTION` reader - 1: RX, error occurred during reception. 0: TX, error occurred during transmission."]
pub type ErrCaptureCodeDirectionR = crate::BitReader;
#[doc = "Field `ERR_CAPTURE_CODE_TYPE` reader - 00: bit error. 01: form error. 10:stuff error. 11:other type of error."]
pub type ErrCaptureCodeTypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - This register contains information about the location of errors on the bus."]
    #[inline(always)]
    pub fn err_capture_code_segment(&self) -> ErrCaptureCodeSegmentR {
        ErrCaptureCodeSegmentR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 1: RX, error occurred during reception. 0: TX, error occurred during transmission."]
    #[inline(always)]
    pub fn err_capture_code_direction(&self) -> ErrCaptureCodeDirectionR {
        ErrCaptureCodeDirectionR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 00: bit error. 01: form error. 10:stuff error. 11:other type of error."]
    #[inline(always)]
    pub fn err_capture_code_type(&self) -> ErrCaptureCodeTypeR {
        ErrCaptureCodeTypeR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "TWAI error info capture register.\n\nYou can [`read`](crate::Reg::read) this register and get [`err_code_cap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrCodeCapSpec;
impl crate::RegisterSpec for ErrCodeCapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_code_cap::R`](R) reader structure"]
impl crate::Readable for ErrCodeCapSpec {}
#[doc = "`reset()` method sets ERR_CODE_CAP to value 0"]
impl crate::Resettable for ErrCodeCapSpec {}

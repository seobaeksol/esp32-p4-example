#[doc = "Register `RC_STATUS1` reader"]
pub type R = crate::R<RcStatus1Spec>;
#[doc = "Field `FRAME_ENC_BITS` reader - Represents all MB actual encoding bits sum value of one frame."]
pub type FrameEncBitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26 - Represents all MB actual encoding bits sum value of one frame."]
    #[inline(always)]
    pub fn frame_enc_bits(&self) -> FrameEncBitsR {
        FrameEncBitsR::new(self.bits & 0x07ff_ffff)
    }
}
#[doc = "Rate control status register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rc_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcStatus1Spec;
impl crate::RegisterSpec for RcStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc_status1::R`](R) reader structure"]
impl crate::Readable for RcStatus1Spec {}
#[doc = "`reset()` method sets RC_STATUS1 to value 0"]
impl crate::Resettable for RcStatus1Spec {}

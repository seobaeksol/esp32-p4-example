#[doc = "Register `FRAME_CODE_LENGTH` reader"]
pub type R = crate::R<FrameCodeLengthSpec>;
#[doc = "Field `FRAME_CODE_LENGTH` reader - Represents current frame code byte length."]
pub type FrameCodeLengthR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Represents current frame code byte length."]
    #[inline(always)]
    pub fn frame_code_length(&self) -> FrameCodeLengthR {
        FrameCodeLengthR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Frame code byte length register.\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_code_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameCodeLengthSpec;
impl crate::RegisterSpec for FrameCodeLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_code_length::R`](R) reader structure"]
impl crate::Readable for FrameCodeLengthSpec {}
#[doc = "`reset()` method sets FRAME_CODE_LENGTH to value 0"]
impl crate::Resettable for FrameCodeLengthSpec {}

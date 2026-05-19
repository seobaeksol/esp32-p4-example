#[doc = "Register `FRAM_NUM` reader"]
pub type R = crate::R<FramNumSpec>;
#[doc = "Field `SOF_FRAME_INDEX` reader - Frame index of received SOF frame."]
pub type SofFrameIndexR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Frame index of received SOF frame."]
    #[inline(always)]
    pub fn sof_frame_index(&self) -> SofFrameIndexR {
        SofFrameIndexR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Last received SOF frame index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fram_num::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramNumSpec;
impl crate::RegisterSpec for FramNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fram_num::R`](R) reader structure"]
impl crate::Readable for FramNumSpec {}
#[doc = "`reset()` method sets FRAM_NUM to value 0"]
impl crate::Resettable for FramNumSpec {}

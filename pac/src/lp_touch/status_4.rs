#[doc = "Register `STATUS_4` reader"]
pub type R = crate::R<Status4Spec>;
#[doc = "Field `PAD4_DATA` reader - need_des"]
pub type Pad4DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD4_DEBOUNCE_CNT` reader - need_des"]
pub type Pad4DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD4_NEG_NOISE_CNT` reader - need_des"]
pub type Pad4NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad4_data(&self) -> Pad4DataR {
        Pad4DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad4_debounce_cnt(&self) -> Pad4DebounceCntR {
        Pad4DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad4_neg_noise_cnt(&self) -> Pad4NegNoiseCntR {
        Pad4NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status4Spec;
impl crate::RegisterSpec for Status4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_4::R`](R) reader structure"]
impl crate::Readable for Status4Spec {}
#[doc = "`reset()` method sets STATUS_4 to value 0"]
impl crate::Resettable for Status4Spec {}

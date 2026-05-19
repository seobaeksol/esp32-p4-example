#[doc = "Register `STATUS_14` reader"]
pub type R = crate::R<Status14Spec>;
#[doc = "Field `PAD14_DATA` reader - need_des"]
pub type Pad14DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD14_DEBOUNCE_CNT` reader - need_des"]
pub type Pad14DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD14_NEG_NOISE_CNT` reader - need_des"]
pub type Pad14NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad14_data(&self) -> Pad14DataR {
        Pad14DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad14_debounce_cnt(&self) -> Pad14DebounceCntR {
        Pad14DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad14_neg_noise_cnt(&self) -> Pad14NegNoiseCntR {
        Pad14NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status14Spec;
impl crate::RegisterSpec for Status14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_14::R`](R) reader structure"]
impl crate::Readable for Status14Spec {}
#[doc = "`reset()` method sets STATUS_14 to value 0"]
impl crate::Resettable for Status14Spec {}

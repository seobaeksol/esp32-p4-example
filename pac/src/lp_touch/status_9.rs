#[doc = "Register `STATUS_9` reader"]
pub type R = crate::R<Status9Spec>;
#[doc = "Field `PAD9_DATA` reader - need_des"]
pub type Pad9DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD9_DEBOUNCE_CNT` reader - need_des"]
pub type Pad9DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD9_NEG_NOISE_CNT` reader - need_des"]
pub type Pad9NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad9_data(&self) -> Pad9DataR {
        Pad9DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad9_debounce_cnt(&self) -> Pad9DebounceCntR {
        Pad9DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad9_neg_noise_cnt(&self) -> Pad9NegNoiseCntR {
        Pad9NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status9Spec;
impl crate::RegisterSpec for Status9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_9::R`](R) reader structure"]
impl crate::Readable for Status9Spec {}
#[doc = "`reset()` method sets STATUS_9 to value 0"]
impl crate::Resettable for Status9Spec {}

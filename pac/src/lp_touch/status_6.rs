#[doc = "Register `STATUS_6` reader"]
pub type R = crate::R<Status6Spec>;
#[doc = "Field `PAD6_DATA` reader - need_des"]
pub type Pad6DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD6_DEBOUNCE_CNT` reader - need_des"]
pub type Pad6DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD6_NEG_NOISE_CNT` reader - need_des"]
pub type Pad6NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad6_data(&self) -> Pad6DataR {
        Pad6DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad6_debounce_cnt(&self) -> Pad6DebounceCntR {
        Pad6DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad6_neg_noise_cnt(&self) -> Pad6NegNoiseCntR {
        Pad6NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status6Spec;
impl crate::RegisterSpec for Status6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_6::R`](R) reader structure"]
impl crate::Readable for Status6Spec {}
#[doc = "`reset()` method sets STATUS_6 to value 0"]
impl crate::Resettable for Status6Spec {}

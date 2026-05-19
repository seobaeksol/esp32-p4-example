#[doc = "Register `STATUS_0` reader"]
pub type R = crate::R<Status0Spec>;
#[doc = "Field `PAD0_DATA` reader - need_des"]
pub type Pad0DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD0_DEBOUNCE_CNT` reader - need_des"]
pub type Pad0DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD0_NEG_NOISE_CNT` reader - need_des"]
pub type Pad0NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad0_data(&self) -> Pad0DataR {
        Pad0DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad0_debounce_cnt(&self) -> Pad0DebounceCntR {
        Pad0DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad0_neg_noise_cnt(&self) -> Pad0NegNoiseCntR {
        Pad0NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status0Spec;
impl crate::RegisterSpec for Status0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_0::R`](R) reader structure"]
impl crate::Readable for Status0Spec {}
#[doc = "`reset()` method sets STATUS_0 to value 0"]
impl crate::Resettable for Status0Spec {}

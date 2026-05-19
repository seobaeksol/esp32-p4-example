#[doc = "Register `STATUS_3` reader"]
pub type R = crate::R<Status3Spec>;
#[doc = "Field `PAD3_DATA` reader - need_des"]
pub type Pad3DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD3_DEBOUNCE_CNT` reader - need_des"]
pub type Pad3DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD3_NEG_NOISE_CNT` reader - need_des"]
pub type Pad3NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad3_data(&self) -> Pad3DataR {
        Pad3DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad3_debounce_cnt(&self) -> Pad3DebounceCntR {
        Pad3DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad3_neg_noise_cnt(&self) -> Pad3NegNoiseCntR {
        Pad3NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status3Spec;
impl crate::RegisterSpec for Status3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_3::R`](R) reader structure"]
impl crate::Readable for Status3Spec {}
#[doc = "`reset()` method sets STATUS_3 to value 0"]
impl crate::Resettable for Status3Spec {}

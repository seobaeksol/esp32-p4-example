#[doc = "Register `STATUS_13` reader"]
pub type R = crate::R<Status13Spec>;
#[doc = "Field `PAD13_DATA` reader - need_des"]
pub type Pad13DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD13_DEBOUNCE_CNT` reader - need_des"]
pub type Pad13DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD13_NEG_NOISE_CNT` reader - need_des"]
pub type Pad13NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad13_data(&self) -> Pad13DataR {
        Pad13DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad13_debounce_cnt(&self) -> Pad13DebounceCntR {
        Pad13DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad13_neg_noise_cnt(&self) -> Pad13NegNoiseCntR {
        Pad13NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status13Spec;
impl crate::RegisterSpec for Status13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_13::R`](R) reader structure"]
impl crate::Readable for Status13Spec {}
#[doc = "`reset()` method sets STATUS_13 to value 0"]
impl crate::Resettable for Status13Spec {}

#[doc = "Register `STATUS_1` reader"]
pub type R = crate::R<Status1Spec>;
#[doc = "Field `PAD1_DATA` reader - need_des"]
pub type Pad1DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD1_DEBOUNCE_CNT` reader - need_des"]
pub type Pad1DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD1_NEG_NOISE_CNT` reader - need_des"]
pub type Pad1NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad1_data(&self) -> Pad1DataR {
        Pad1DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad1_debounce_cnt(&self) -> Pad1DebounceCntR {
        Pad1DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad1_neg_noise_cnt(&self) -> Pad1NegNoiseCntR {
        Pad1NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status1Spec;
impl crate::RegisterSpec for Status1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_1::R`](R) reader structure"]
impl crate::Readable for Status1Spec {}
#[doc = "`reset()` method sets STATUS_1 to value 0"]
impl crate::Resettable for Status1Spec {}

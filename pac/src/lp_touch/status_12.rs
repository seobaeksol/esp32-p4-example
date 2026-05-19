#[doc = "Register `STATUS_12` reader"]
pub type R = crate::R<Status12Spec>;
#[doc = "Field `PAD12_DATA` reader - need_des"]
pub type Pad12DataR = crate::FieldReader<u16>;
#[doc = "Field `PAD12_DEBOUNCE_CNT` reader - need_des"]
pub type Pad12DebounceCntR = crate::FieldReader;
#[doc = "Field `PAD12_NEG_NOISE_CNT` reader - need_des"]
pub type Pad12NegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad12_data(&self) -> Pad12DataR {
        Pad12DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad12_debounce_cnt(&self) -> Pad12DebounceCntR {
        Pad12DebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad12_neg_noise_cnt(&self) -> Pad12NegNoiseCntR {
        Pad12NegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status12Spec;
impl crate::RegisterSpec for Status12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_12::R`](R) reader structure"]
impl crate::Readable for Status12Spec {}
#[doc = "`reset()` method sets STATUS_12 to value 0"]
impl crate::Resettable for Status12Spec {}

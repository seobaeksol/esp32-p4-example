#[doc = "Register `STATUS_15` reader"]
pub type R = crate::R<Status15Spec>;
#[doc = "Field `SLP_DATA` reader - need_des"]
pub type SlpDataR = crate::FieldReader<u16>;
#[doc = "Field `SLP_DEBOUNCE_CNT` reader - need_des"]
pub type SlpDebounceCntR = crate::FieldReader;
#[doc = "Field `SLP_NEG_NOISE_CNT` reader - need_des"]
pub type SlpNegNoiseCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn slp_data(&self) -> SlpDataR {
        SlpDataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn slp_debounce_cnt(&self) -> SlpDebounceCntR {
        SlpDebounceCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn slp_neg_noise_cnt(&self) -> SlpNegNoiseCntR {
        SlpNegNoiseCntR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status15Spec;
impl crate::RegisterSpec for Status15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_15::R`](R) reader structure"]
impl crate::Readable for Status15Spec {}
#[doc = "`reset()` method sets STATUS_15 to value 0"]
impl crate::Resettable for Status15Spec {}

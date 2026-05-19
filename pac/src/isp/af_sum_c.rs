#[doc = "Register `AF_SUM_C` reader"]
pub type R = crate::R<AfSumCSpec>;
#[doc = "Field `AF_SUMC` reader - this field represents the result of accumulation of pix grad of focus window c"]
pub type AfSumcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - this field represents the result of accumulation of pix grad of focus window c"]
    #[inline(always)]
    pub fn af_sumc(&self) -> AfSumcR {
        AfSumcR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "result of sum of af window c\n\nYou can [`read`](crate::Reg::read) this register and get [`af_sum_c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfSumCSpec;
impl crate::RegisterSpec for AfSumCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_sum_c::R`](R) reader structure"]
impl crate::Readable for AfSumCSpec {}
#[doc = "`reset()` method sets AF_SUM_C to value 0"]
impl crate::Resettable for AfSumCSpec {}

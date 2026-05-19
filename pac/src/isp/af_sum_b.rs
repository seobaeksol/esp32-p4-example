#[doc = "Register `AF_SUM_B` reader"]
pub type R = crate::R<AfSumBSpec>;
#[doc = "Field `AF_SUMB` reader - this field represents the result of accumulation of pix grad of focus window b"]
pub type AfSumbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - this field represents the result of accumulation of pix grad of focus window b"]
    #[inline(always)]
    pub fn af_sumb(&self) -> AfSumbR {
        AfSumbR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "result of sum of af window b\n\nYou can [`read`](crate::Reg::read) this register and get [`af_sum_b::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfSumBSpec;
impl crate::RegisterSpec for AfSumBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_sum_b::R`](R) reader structure"]
impl crate::Readable for AfSumBSpec {}
#[doc = "`reset()` method sets AF_SUM_B to value 0"]
impl crate::Resettable for AfSumBSpec {}

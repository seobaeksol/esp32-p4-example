#[doc = "Register `AF_LUM_C` reader"]
pub type R = crate::R<AfLumCSpec>;
#[doc = "Field `AF_LUMC` reader - this field represents the result of accumulation of pix light of focus window c"]
pub type AfLumcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - this field represents the result of accumulation of pix light of focus window c"]
    #[inline(always)]
    pub fn af_lumc(&self) -> AfLumcR {
        AfLumcR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "result of lum of af window c\n\nYou can [`read`](crate::Reg::read) this register and get [`af_lum_c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfLumCSpec;
impl crate::RegisterSpec for AfLumCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_lum_c::R`](R) reader structure"]
impl crate::Readable for AfLumCSpec {}
#[doc = "`reset()` method sets AF_LUM_C to value 0"]
impl crate::Resettable for AfLumCSpec {}

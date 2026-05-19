#[doc = "Register `QUERY_CLEAN` reader"]
pub type R = crate::R<QueryCleanSpec>;
#[doc = "Field `QUERY_CLEAN` reader - Represents whether or not the RSA memory completes initialization.\\\\ 0: Not complete\\\\ 1: Completed\\\\"]
pub type QueryCleanR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not the RSA memory completes initialization.\\\\ 0: Not complete\\\\ 1: Completed\\\\"]
    #[inline(always)]
    pub fn query_clean(&self) -> QueryCleanR {
        QueryCleanR::new((self.bits & 1) != 0)
    }
}
#[doc = "RSA initialization status\n\nYou can [`read`](crate::Reg::read) this register and get [`query_clean::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QueryCleanSpec;
impl crate::RegisterSpec for QueryCleanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_clean::R`](R) reader structure"]
impl crate::Readable for QueryCleanSpec {}
#[doc = "`reset()` method sets QUERY_CLEAN to value 0"]
impl crate::Resettable for QueryCleanSpec {}

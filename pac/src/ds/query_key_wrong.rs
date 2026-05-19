#[doc = "Register `QUERY_KEY_WRONG` reader"]
pub type R = crate::R<QueryKeyWrongSpec>;
#[doc = "Field `QUERY_KEY_WRONG` reader - Represents the specific problem with HMAC initialization.\\\\ 0: HMAC is not called\\\\ 1-15: HMAC was activated, but the DS peripheral did not successfully receive the \\begin{math}DS_KEY\\end{math} from the HMAC peripheral. (The biggest value is 15)\\\\"]
pub type QueryKeyWrongR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents the specific problem with HMAC initialization.\\\\ 0: HMAC is not called\\\\ 1-15: HMAC was activated, but the DS peripheral did not successfully receive the \\begin{math}DS_KEY\\end{math} from the HMAC peripheral. (The biggest value is 15)\\\\"]
    #[inline(always)]
    pub fn query_key_wrong(&self) -> QueryKeyWrongR {
        QueryKeyWrongR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Checks the reason why \\begin{math}DS_KEY\\end{math} is not ready\n\nYou can [`read`](crate::Reg::read) this register and get [`query_key_wrong::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QueryKeyWrongSpec;
impl crate::RegisterSpec for QueryKeyWrongSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_key_wrong::R`](R) reader structure"]
impl crate::Readable for QueryKeyWrongSpec {}
#[doc = "`reset()` method sets QUERY_KEY_WRONG to value 0"]
impl crate::Resettable for QueryKeyWrongSpec {}

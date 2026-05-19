#[doc = "Register `INTR_SRC_PASS_IN_SEC_STATUS_2` reader"]
pub type R = crate::R<IntrSrcPassInSecStatus2Spec>;
#[doc = "Field `CORE1_INTR_SRC_PASS_IN_SEC_STATUS_2` reader - NA"]
pub type Core1IntrSrcPassInSecStatus2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core1_intr_src_pass_in_sec_status_2(&self) -> Core1IntrSrcPassInSecStatus2R {
        Core1IntrSrcPassInSecStatus2R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSrcPassInSecStatus2Spec;
impl crate::RegisterSpec for IntrSrcPassInSecStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_src_pass_in_sec_status_2::R`](R) reader structure"]
impl crate::Readable for IntrSrcPassInSecStatus2Spec {}
#[doc = "`reset()` method sets INTR_SRC_PASS_IN_SEC_STATUS_2 to value 0"]
impl crate::Resettable for IntrSrcPassInSecStatus2Spec {}

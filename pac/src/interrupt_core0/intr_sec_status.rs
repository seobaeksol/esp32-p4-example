#[doc = "Register `INTR_SEC_STATUS` reader"]
pub type R = crate::R<IntrSecStatusSpec>;
#[doc = "Field `CORE0_INTR_SEC_STATUS` reader - NA"]
pub type Core0IntrSecStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core0_intr_sec_status(&self) -> Core0IntrSecStatusR {
        Core0IntrSecStatusR::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sec_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSecStatusSpec;
impl crate::RegisterSpec for IntrSecStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sec_status::R`](R) reader structure"]
impl crate::Readable for IntrSecStatusSpec {}
#[doc = "`reset()` method sets INTR_SEC_STATUS to value 0"]
impl crate::Resettable for IntrSecStatusSpec {}

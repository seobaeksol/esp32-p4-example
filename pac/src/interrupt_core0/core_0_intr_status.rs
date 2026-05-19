#[doc = "Register `CORE_0_INTR_STATUS%s` reader"]
pub type R = crate::R<Core0IntrStatusSpec>;
#[doc = "Field `CORE0_INTR_STATUS_0` reader - NA"]
pub type Core0IntrStatus0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core0_intr_status_0(&self) -> Core0IntrStatus0R {
        Core0IntrStatus0R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0IntrStatusSpec;
impl crate::RegisterSpec for Core0IntrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_status::R`](R) reader structure"]
impl crate::Readable for Core0IntrStatusSpec {}
#[doc = "`reset()` method sets CORE_0_INTR_STATUS%s to value 0"]
impl crate::Resettable for Core0IntrStatusSpec {}

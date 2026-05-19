#[doc = "Register `STATUS_NEXT` reader"]
pub type R = crate::R<StatusNextSpec>;
#[doc = "Field `STATUS_INTERRUPT_NEXT` reader - GPIO interrupt source register for GPIO0-31"]
pub type StatusInterruptNextR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt source register for GPIO0-31"]
    #[inline(always)]
    pub fn status_interrupt_next(&self) -> StatusInterruptNextR {
        StatusInterruptNextR::new(self.bits)
    }
}
#[doc = "GPIO interrupt source register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusNextSpec;
impl crate::RegisterSpec for StatusNextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_next::R`](R) reader structure"]
impl crate::Readable for StatusNextSpec {}
#[doc = "`reset()` method sets STATUS_NEXT to value 0"]
impl crate::Resettable for StatusNextSpec {}

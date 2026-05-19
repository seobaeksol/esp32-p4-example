#[doc = "Register `INTR_2` reader"]
pub type R = crate::R<Intr2Spec>;
#[doc = "Field `INT_2` reader - GPIO interrupt 2 status register for GPIO0-31"]
pub type Int2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt 2 status register for GPIO0-31"]
    #[inline(always)]
    pub fn int_2(&self) -> Int2R {
        Int2R::new(self.bits)
    }
}
#[doc = "GPIO interrupt 2 status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intr2Spec;
impl crate::RegisterSpec for Intr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_2::R`](R) reader structure"]
impl crate::Readable for Intr2Spec {}
#[doc = "`reset()` method sets INTR_2 to value 0"]
impl crate::Resettable for Intr2Spec {}

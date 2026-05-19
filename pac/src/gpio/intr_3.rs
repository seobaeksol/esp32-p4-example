#[doc = "Register `INTR_3` reader"]
pub type R = crate::R<Intr3Spec>;
#[doc = "Field `INT_3` reader - GPIO interrupt 3 status register for GPIO0-31"]
pub type Int3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt 3 status register for GPIO0-31"]
    #[inline(always)]
    pub fn int_3(&self) -> Int3R {
        Int3R::new(self.bits)
    }
}
#[doc = "GPIO interrupt 3 status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intr3Spec;
impl crate::RegisterSpec for Intr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_3::R`](R) reader structure"]
impl crate::Readable for Intr3Spec {}
#[doc = "`reset()` method sets INTR_3 to value 0"]
impl crate::Resettable for Intr3Spec {}

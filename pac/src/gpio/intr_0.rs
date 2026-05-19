#[doc = "Register `INTR_0` reader"]
pub type R = crate::R<Intr0Spec>;
#[doc = "Field `INT_0` reader - GPIO interrupt 0 status register for GPIO0-31"]
pub type Int0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt 0 status register for GPIO0-31"]
    #[inline(always)]
    pub fn int_0(&self) -> Int0R {
        Int0R::new(self.bits)
    }
}
#[doc = "GPIO interrupt 0 status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intr0Spec;
impl crate::RegisterSpec for Intr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_0::R`](R) reader structure"]
impl crate::Readable for Intr0Spec {}
#[doc = "`reset()` method sets INTR_0 to value 0"]
impl crate::Resettable for Intr0Spec {}

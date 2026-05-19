#[doc = "Register `INTR1_1` reader"]
pub type R = crate::R<Intr1_1Spec>;
#[doc = "Field `INT1_1` reader - GPIO interrupt 1 status register for GPIO32-56"]
pub type Int1_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO interrupt 1 status register for GPIO32-56"]
    #[inline(always)]
    pub fn int1_1(&self) -> Int1_1R {
        Int1_1R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "GPIO interrupt 1 status register for GPIO32-56\n\nYou can [`read`](crate::Reg::read) this register and get [`intr1_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intr1_1Spec;
impl crate::RegisterSpec for Intr1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_1::R`](R) reader structure"]
impl crate::Readable for Intr1_1Spec {}
#[doc = "`reset()` method sets INTR1_1 to value 0"]
impl crate::Resettable for Intr1_1Spec {}

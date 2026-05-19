#[doc = "Register `INTR1_2` reader"]
pub type R = crate::R<Intr1_2Spec>;
#[doc = "Field `INT1_2` reader - GPIO interrupt 2 status register for GPIO32-56"]
pub type Int1_2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO interrupt 2 status register for GPIO32-56"]
    #[inline(always)]
    pub fn int1_2(&self) -> Int1_2R {
        Int1_2R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "GPIO interrupt 2 status register for GPIO32-56\n\nYou can [`read`](crate::Reg::read) this register and get [`intr1_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intr1_2Spec;
impl crate::RegisterSpec for Intr1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_2::R`](R) reader structure"]
impl crate::Readable for Intr1_2Spec {}
#[doc = "`reset()` method sets INTR1_2 to value 0"]
impl crate::Resettable for Intr1_2Spec {}

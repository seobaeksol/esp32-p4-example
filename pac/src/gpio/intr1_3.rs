#[doc = "Register `INTR1_3` reader"]
pub type R = crate::R<Intr1_3Spec>;
#[doc = "Field `INT1_3` reader - GPIO interrupt 3 status register for GPIO32-56"]
pub type Int1_3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO interrupt 3 status register for GPIO32-56"]
    #[inline(always)]
    pub fn int1_3(&self) -> Int1_3R {
        Int1_3R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "GPIO interrupt 3 status register for GPIO32-56\n\nYou can [`read`](crate::Reg::read) this register and get [`intr1_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intr1_3Spec;
impl crate::RegisterSpec for Intr1_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_3::R`](R) reader structure"]
impl crate::Readable for Intr1_3Spec {}
#[doc = "`reset()` method sets INTR1_3 to value 0"]
impl crate::Resettable for Intr1_3Spec {}

#[doc = "Register `INTR1_0` reader"]
pub type R = crate::R<Intr1_0Spec>;
#[doc = "Field `INT1_0` reader - GPIO interrupt 0 status register for GPIO32-56"]
pub type Int1_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO interrupt 0 status register for GPIO32-56"]
    #[inline(always)]
    pub fn int1_0(&self) -> Int1_0R {
        Int1_0R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "GPIO interrupt 0 status register for GPIO32-56\n\nYou can [`read`](crate::Reg::read) this register and get [`intr1_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intr1_0Spec;
impl crate::RegisterSpec for Intr1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_0::R`](R) reader structure"]
impl crate::Readable for Intr1_0Spec {}
#[doc = "`reset()` method sets INTR1_0 to value 0"]
impl crate::Resettable for Intr1_0Spec {}

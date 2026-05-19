#[doc = "Register `IN1` reader"]
pub type R = crate::R<In1Spec>;
#[doc = "Field `DATA_NEXT` reader - GPIO input register for GPIO32-56"]
pub type DataNextR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO input register for GPIO32-56"]
    #[inline(always)]
    pub fn data_next(&self) -> DataNextR {
        DataNextR::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "GPIO input register for GPIO32-56\n\nYou can [`read`](crate::Reg::read) this register and get [`in1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct In1Spec;
impl crate::RegisterSpec for In1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in1::R`](R) reader structure"]
impl crate::Readable for In1Spec {}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for In1Spec {}

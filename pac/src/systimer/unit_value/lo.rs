#[doc = "Register `LO` reader"]
pub type R = crate::R<LoSpec>;
#[doc = "Field `VALUE_LO` reader - timer read value low 32bits"]
pub type ValueLoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - timer read value low 32bits"]
    #[inline(always)]
    pub fn value_lo(&self) -> ValueLoR {
        ValueLoR::new(self.bits)
    }
}
#[doc = "system timer unit0 value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSpec;
impl crate::RegisterSpec for LoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LoSpec {}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LoSpec {}

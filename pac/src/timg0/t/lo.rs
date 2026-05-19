#[doc = "Register `LO` reader"]
pub type R = crate::R<LoSpec>;
#[doc = "Field `LO` reader - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
pub type LoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new(self.bits)
    }
}
#[doc = "Timer 0 current value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSpec;
impl crate::RegisterSpec for LoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LoSpec {}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LoSpec {}

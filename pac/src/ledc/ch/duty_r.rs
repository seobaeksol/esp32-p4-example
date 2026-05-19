#[doc = "Register `DUTY_R` reader"]
pub type R = crate::R<DutyRSpec>;
#[doc = "Field `DUTY_R` reader - Represents the current duty of output signal on channel %s."]
pub type DutyRR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Represents the current duty of output signal on channel %s."]
    #[inline(always)]
    pub fn duty_r(&self) -> DutyRR {
        DutyRR::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "Current duty cycle register for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`duty_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DutyRSpec;
impl crate::RegisterSpec for DutyRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duty_r::R`](R) reader structure"]
impl crate::Readable for DutyRSpec {}
#[doc = "`reset()` method sets DUTY_R to value 0"]
impl crate::Resettable for DutyRSpec {}

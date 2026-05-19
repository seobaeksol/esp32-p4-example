#[doc = "Register `MEAS_STATUS` reader"]
pub type R = crate::R<MeasStatusSpec>;
#[doc = "Field `SARADC_MEAS_STATUS` reader - N/A"]
pub type SaradcMeasStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn saradc_meas_status(&self) -> SaradcMeasStatusR {
        SaradcMeasStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`meas_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeasStatusSpec;
impl crate::RegisterSpec for MeasStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meas_status::R`](R) reader structure"]
impl crate::Readable for MeasStatusSpec {}
#[doc = "`reset()` method sets MEAS_STATUS to value 0"]
impl crate::Resettable for MeasStatusSpec {}

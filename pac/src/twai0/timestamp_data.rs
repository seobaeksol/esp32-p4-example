#[doc = "Register `TIMESTAMP_DATA` reader"]
pub type R = crate::R<TimestampDataSpec>;
#[doc = "Field `TIMESTAMP_DATA` reader - Data of timestamp of a CAN frame."]
pub type TimestampDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data of timestamp of a CAN frame."]
    #[inline(always)]
    pub fn timestamp_data(&self) -> TimestampDataR {
        TimestampDataR::new(self.bits)
    }
}
#[doc = "Timestamp data register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampDataSpec;
impl crate::RegisterSpec for TimestampDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_data::R`](R) reader structure"]
impl crate::Readable for TimestampDataSpec {}
#[doc = "`reset()` method sets TIMESTAMP_DATA to value 0"]
impl crate::Resettable for TimestampDataSpec {}

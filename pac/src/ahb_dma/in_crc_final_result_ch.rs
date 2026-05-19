#[doc = "Register `IN_CRC_FINAL_RESULT_CH%s` reader"]
pub type R = crate::R<InCrcFinalResultChSpec>;
#[doc = "Field `IN_CRC_FINAL_RESULT_CH` reader - This register is used to store result ch0 of rx"]
pub type InCrcFinalResultChR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to store result ch0 of rx"]
    #[inline(always)]
    pub fn in_crc_final_result_ch(&self) -> InCrcFinalResultChR {
        InCrcFinalResultChR::new(self.bits)
    }
}
#[doc = "This register is used to store ch%s crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_final_result_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCrcFinalResultChSpec;
impl crate::RegisterSpec for InCrcFinalResultChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_crc_final_result_ch::R`](R) reader structure"]
impl crate::Readable for InCrcFinalResultChSpec {}
#[doc = "`reset()` method sets IN_CRC_FINAL_RESULT_CH%s to value 0"]
impl crate::Resettable for InCrcFinalResultChSpec {}

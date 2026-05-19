#[doc = "Register `OUT_CRC_FINAL_RESULT_CH%s` reader"]
pub type R = crate::R<OutCrcFinalResultChSpec>;
#[doc = "Field `OUT_CRC_FINAL_RESULT_CH` reader - This register is used to store result ch0 of tx"]
pub type OutCrcFinalResultChR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to store result ch0 of tx"]
    #[inline(always)]
    pub fn out_crc_final_result_ch(&self) -> OutCrcFinalResultChR {
        OutCrcFinalResultChR::new(self.bits)
    }
}
#[doc = "This register is used to store ch%s crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_final_result_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCrcFinalResultChSpec;
impl crate::RegisterSpec for OutCrcFinalResultChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_crc_final_result_ch::R`](R) reader structure"]
impl crate::Readable for OutCrcFinalResultChSpec {}
#[doc = "`reset()` method sets OUT_CRC_FINAL_RESULT_CH%s to value 0"]
impl crate::Resettable for OutCrcFinalResultChSpec {}

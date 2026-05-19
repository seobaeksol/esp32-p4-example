#[doc = "Register `POP_DATA_CNT` reader"]
pub type R = crate::R<PopDataCntSpec>;
#[doc = "Field `IN_CMDFIFO_POP_DATA_CNT` reader - only for debug"]
pub type InCmdfifoPopDataCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_pop_data_cnt(&self) -> InCmdfifoPopDataCntR {
        InCmdfifoPopDataCntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX CHx pop data cnt register\n\nYou can [`read`](crate::Reg::read) this register and get [`pop_data_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PopDataCntSpec;
impl crate::RegisterSpec for PopDataCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pop_data_cnt::R`](R) reader structure"]
impl crate::Readable for PopDataCntSpec {}
#[doc = "`reset()` method sets POP_DATA_CNT to value 0x07"]
impl crate::Resettable for PopDataCntSpec {
    const RESET_VALUE: u32 = 0x07;
}

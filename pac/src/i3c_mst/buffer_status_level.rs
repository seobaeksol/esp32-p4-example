#[doc = "Register `BUFFER_STATUS_LEVEL` reader"]
pub type R = crate::R<BufferStatusLevelSpec>;
#[doc = "Field `CMD_BUF_EMPTY_CNT` reader - Command Buffer Empty Locations contains the number of empty locations in the command buffer."]
pub type CmdBufEmptyCntR = crate::FieldReader;
#[doc = "Field `RESP_BUF_CNT` reader - Response Buffer Level Value contains the number of valid data entries in the response buffer."]
pub type RespBufCntR = crate::FieldReader;
#[doc = "Field `IBI_DATA_BUF_CNT` reader - IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode."]
pub type IbiDataBufCntR = crate::FieldReader;
#[doc = "Field `IBI_STATUS_BUF_CNT` reader - IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode."]
pub type IbiStatusBufCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Command Buffer Empty Locations contains the number of empty locations in the command buffer."]
    #[inline(always)]
    pub fn cmd_buf_empty_cnt(&self) -> CmdBufEmptyCntR {
        CmdBufEmptyCntR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Response Buffer Level Value contains the number of valid data entries in the response buffer."]
    #[inline(always)]
    pub fn resp_buf_cnt(&self) -> RespBufCntR {
        RespBufCntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode."]
    #[inline(always)]
    pub fn ibi_data_buf_cnt(&self) -> IbiDataBufCntR {
        IbiDataBufCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode."]
    #[inline(always)]
    pub fn ibi_status_buf_cnt(&self) -> IbiStatusBufCntR {
        IbiStatusBufCntR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "BUFFER_STATUS_LEVEL reflects the status level of Buffers in the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`buffer_status_level::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufferStatusLevelSpec;
impl crate::RegisterSpec for BufferStatusLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buffer_status_level::R`](R) reader structure"]
impl crate::Readable for BufferStatusLevelSpec {}
#[doc = "`reset()` method sets BUFFER_STATUS_LEVEL to value 0x10"]
impl crate::Resettable for BufferStatusLevelSpec {
    const RESET_VALUE: u32 = 0x10;
}

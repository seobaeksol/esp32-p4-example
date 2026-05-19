#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `TX_DATA_BUF_THLD` reader - This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value."]
pub type TxDataBufThldR = crate::BitReader;
#[doc = "Field `RX_DATA_BUF_THLD` reader - This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value."]
pub type RxDataBufThldR = crate::BitReader;
#[doc = "Field `IBI_STATUS_THLD` reader - Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value."]
pub type IbiStatusThldR = crate::BitReader;
#[doc = "Field `CMD_BUF_EMPTY_THLD` reader - This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value."]
pub type CmdBufEmptyThldR = crate::BitReader;
#[doc = "Field `RESP_READY` reader - This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value."]
pub type RespReadyR = crate::BitReader;
#[doc = "Field `NXT_CMD_REQ_ERR` reader - This interrupt is generated if toc is 0(master will restart next command), but command buf is empty."]
pub type NxtCmdReqErrR = crate::BitReader;
#[doc = "Field `TRANSFER_ERR` reader - This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1."]
pub type TransferErrR = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE` reader - NA"]
pub type TransferCompleteR = crate::BitReader;
#[doc = "Field `COMMAND_DONE` reader - NA"]
pub type CommandDoneR = crate::BitReader;
#[doc = "Field `DETECT_START` reader - NA"]
pub type DetectStartR = crate::BitReader;
#[doc = "Field `RESP_BUF_OVF` reader - NA"]
pub type RespBufOvfR = crate::BitReader;
#[doc = "Field `IBI_DATA_BUF_OVF` reader - NA"]
pub type IbiDataBufOvfR = crate::BitReader;
#[doc = "Field `IBI_STATUS_BUF_OVF` reader - NA"]
pub type IbiStatusBufOvfR = crate::BitReader;
#[doc = "Field `IBI_HANDLE_DONE` reader - NA"]
pub type IbiHandleDoneR = crate::BitReader;
#[doc = "Field `IBI_DETECT` reader - NA"]
pub type IbiDetectR = crate::BitReader;
#[doc = "Field `CMD_CCC_MISMATCH` reader - NA"]
pub type CmdCccMismatchR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value."]
    #[inline(always)]
    pub fn tx_data_buf_thld(&self) -> TxDataBufThldR {
        TxDataBufThldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value."]
    #[inline(always)]
    pub fn rx_data_buf_thld(&self) -> RxDataBufThldR {
        RxDataBufThldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value."]
    #[inline(always)]
    pub fn ibi_status_thld(&self) -> IbiStatusThldR {
        IbiStatusThldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value."]
    #[inline(always)]
    pub fn cmd_buf_empty_thld(&self) -> CmdBufEmptyThldR {
        CmdBufEmptyThldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value."]
    #[inline(always)]
    pub fn resp_ready(&self) -> RespReadyR {
        RespReadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt is generated if toc is 0(master will restart next command), but command buf is empty."]
    #[inline(always)]
    pub fn nxt_cmd_req_err(&self) -> NxtCmdReqErrR {
        NxtCmdReqErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1."]
    #[inline(always)]
    pub fn transfer_err(&self) -> TransferErrR {
        TransferErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete(&self) -> TransferCompleteR {
        TransferCompleteR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done(&self) -> CommandDoneR {
        CommandDoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start(&self) -> DetectStartR {
        DetectStartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf(&self) -> RespBufOvfR {
        RespBufOvfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf(&self) -> IbiDataBufOvfR {
        IbiDataBufOvfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf(&self) -> IbiStatusBufOvfR {
        IbiStatusBufOvfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done(&self) -> IbiHandleDoneR {
        IbiHandleDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect(&self) -> IbiDetectR {
        IbiDetectR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch(&self) -> CmdCccMismatchR {
        CmdCccMismatchR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}

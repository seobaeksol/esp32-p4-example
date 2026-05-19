#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `TX_DATA_BUF_THLD` writer - NA"]
pub type TxDataBufThldW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_DATA_BUF_THLD` writer - NA"]
pub type RxDataBufThldW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_STATUS_THLD` writer - NA"]
pub type IbiStatusThldW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_BUF_EMPTY_THLD` writer - NA"]
pub type CmdBufEmptyThldW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESP_READY` writer - NA"]
pub type RespReadyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NXT_CMD_REQ_ERR` writer - NA"]
pub type NxtCmdReqErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANSFER_ERR` writer - NA"]
pub type TransferErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANSFER_COMPLETE` writer - NA"]
pub type TransferCompleteW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMMAND_DONE` writer - NA"]
pub type CommandDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DETECT_START` writer - NA"]
pub type DetectStartW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESP_BUF_OVF` writer - NA"]
pub type RespBufOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_DATA_BUF_OVF` writer - NA"]
pub type IbiDataBufOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_STATUS_BUF_OVF` writer - NA"]
pub type IbiStatusBufOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_HANDLE_DONE` writer - NA"]
pub type IbiHandleDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_DETECT` writer - NA"]
pub type IbiDetectW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_CCC_MISMATCH` writer - NA"]
pub type CmdCccMismatchW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn tx_data_buf_thld(&mut self) -> TxDataBufThldW<'_, IntClrSpec> {
        TxDataBufThldW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn rx_data_buf_thld(&mut self) -> RxDataBufThldW<'_, IntClrSpec> {
        RxDataBufThldW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ibi_status_thld(&mut self) -> IbiStatusThldW<'_, IntClrSpec> {
        IbiStatusThldW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn cmd_buf_empty_thld(&mut self) -> CmdBufEmptyThldW<'_, IntClrSpec> {
        CmdBufEmptyThldW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn resp_ready(&mut self) -> RespReadyW<'_, IntClrSpec> {
        RespReadyW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn nxt_cmd_req_err(&mut self) -> NxtCmdReqErrW<'_, IntClrSpec> {
        NxtCmdReqErrW::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn transfer_err(&mut self) -> TransferErrW<'_, IntClrSpec> {
        TransferErrW::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete(&mut self) -> TransferCompleteW<'_, IntClrSpec> {
        TransferCompleteW::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done(&mut self) -> CommandDoneW<'_, IntClrSpec> {
        CommandDoneW::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start(&mut self) -> DetectStartW<'_, IntClrSpec> {
        DetectStartW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf(&mut self) -> RespBufOvfW<'_, IntClrSpec> {
        RespBufOvfW::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf(&mut self) -> IbiDataBufOvfW<'_, IntClrSpec> {
        IbiDataBufOvfW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf(&mut self) -> IbiStatusBufOvfW<'_, IntClrSpec> {
        IbiStatusBufOvfW::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done(&mut self) -> IbiHandleDoneW<'_, IntClrSpec> {
        IbiHandleDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect(&mut self) -> IbiDetectW<'_, IntClrSpec> {
        IbiDetectW::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch(&mut self) -> CmdCccMismatchW<'_, IntClrSpec> {
        CmdCccMismatchW::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}

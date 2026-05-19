#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `TX_DATA_BUF_THLD` reader - NA"]
pub type TxDataBufThldR = crate::BitReader;
#[doc = "Field `TX_DATA_BUF_THLD` writer - NA"]
pub type TxDataBufThldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_BUF_THLD` reader - NA"]
pub type RxDataBufThldR = crate::BitReader;
#[doc = "Field `RX_DATA_BUF_THLD` writer - NA"]
pub type RxDataBufThldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_THLD` reader - NA"]
pub type IbiStatusThldR = crate::BitReader;
#[doc = "Field `IBI_STATUS_THLD` writer - NA"]
pub type IbiStatusThldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_BUF_EMPTY_THLD` reader - NA"]
pub type CmdBufEmptyThldR = crate::BitReader;
#[doc = "Field `CMD_BUF_EMPTY_THLD` writer - NA"]
pub type CmdBufEmptyThldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_READY` reader - NA"]
pub type RespReadyR = crate::BitReader;
#[doc = "Field `RESP_READY` writer - NA"]
pub type RespReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NXT_CMD_REQ_ERR` reader - NA"]
pub type NxtCmdReqErrR = crate::BitReader;
#[doc = "Field `NXT_CMD_REQ_ERR` writer - NA"]
pub type NxtCmdReqErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_ERR` reader - NA"]
pub type TransferErrR = crate::BitReader;
#[doc = "Field `TRANSFER_ERR` writer - NA"]
pub type TransferErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_COMPLETE` reader - NA"]
pub type TransferCompleteR = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE` writer - NA"]
pub type TransferCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_DONE` reader - NA"]
pub type CommandDoneR = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - NA"]
pub type CommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETECT_START` reader - NA"]
pub type DetectStartR = crate::BitReader;
#[doc = "Field `DETECT_START` writer - NA"]
pub type DetectStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_BUF_OVF` reader - NA"]
pub type RespBufOvfR = crate::BitReader;
#[doc = "Field `RESP_BUF_OVF` writer - NA"]
pub type RespBufOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DATA_BUF_OVF` reader - NA"]
pub type IbiDataBufOvfR = crate::BitReader;
#[doc = "Field `IBI_DATA_BUF_OVF` writer - NA"]
pub type IbiDataBufOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_BUF_OVF` reader - NA"]
pub type IbiStatusBufOvfR = crate::BitReader;
#[doc = "Field `IBI_STATUS_BUF_OVF` writer - NA"]
pub type IbiStatusBufOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_HANDLE_DONE` reader - NA"]
pub type IbiHandleDoneR = crate::BitReader;
#[doc = "Field `IBI_HANDLE_DONE` writer - NA"]
pub type IbiHandleDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DETECT` reader - NA"]
pub type IbiDetectR = crate::BitReader;
#[doc = "Field `IBI_DETECT` writer - NA"]
pub type IbiDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CCC_MISMATCH` reader - NA"]
pub type CmdCccMismatchR = crate::BitReader;
#[doc = "Field `CMD_CCC_MISMATCH` writer - NA"]
pub type CmdCccMismatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn tx_data_buf_thld(&self) -> TxDataBufThldR {
        TxDataBufThldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn rx_data_buf_thld(&self) -> RxDataBufThldR {
        RxDataBufThldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ibi_status_thld(&self) -> IbiStatusThldR {
        IbiStatusThldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn cmd_buf_empty_thld(&self) -> CmdBufEmptyThldR {
        CmdBufEmptyThldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn resp_ready(&self) -> RespReadyR {
        RespReadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn nxt_cmd_req_err(&self) -> NxtCmdReqErrR {
        NxtCmdReqErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
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
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn tx_data_buf_thld(&mut self) -> TxDataBufThldW<'_, IntRawSpec> {
        TxDataBufThldW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn rx_data_buf_thld(&mut self) -> RxDataBufThldW<'_, IntRawSpec> {
        RxDataBufThldW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ibi_status_thld(&mut self) -> IbiStatusThldW<'_, IntRawSpec> {
        IbiStatusThldW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn cmd_buf_empty_thld(&mut self) -> CmdBufEmptyThldW<'_, IntRawSpec> {
        CmdBufEmptyThldW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn resp_ready(&mut self) -> RespReadyW<'_, IntRawSpec> {
        RespReadyW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn nxt_cmd_req_err(&mut self) -> NxtCmdReqErrW<'_, IntRawSpec> {
        NxtCmdReqErrW::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn transfer_err(&mut self) -> TransferErrW<'_, IntRawSpec> {
        TransferErrW::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete(&mut self) -> TransferCompleteW<'_, IntRawSpec> {
        TransferCompleteW::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done(&mut self) -> CommandDoneW<'_, IntRawSpec> {
        CommandDoneW::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start(&mut self) -> DetectStartW<'_, IntRawSpec> {
        DetectStartW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf(&mut self) -> RespBufOvfW<'_, IntRawSpec> {
        RespBufOvfW::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf(&mut self) -> IbiDataBufOvfW<'_, IntRawSpec> {
        IbiDataBufOvfW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf(&mut self) -> IbiStatusBufOvfW<'_, IntRawSpec> {
        IbiStatusBufOvfW::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done(&mut self) -> IbiHandleDoneW<'_, IntRawSpec> {
        IbiHandleDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect(&mut self) -> IbiDetectW<'_, IntRawSpec> {
        IbiDetectW::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch(&mut self) -> CmdCccMismatchW<'_, IntRawSpec> {
        CmdCccMismatchW::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0x08"]
impl crate::Resettable for IntRawSpec {
    const RESET_VALUE: u32 = 0x08;
}

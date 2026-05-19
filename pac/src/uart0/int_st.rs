#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `RXFIFO_FULL` reader - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
pub type RxfifoFullR = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` reader - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
pub type TxfifoEmptyR = crate::BitReader;
#[doc = "Field `PARITY_ERR` reader - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
pub type ParityErrR = crate::BitReader;
#[doc = "Field `FRM_ERR` reader - This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1."]
pub type FrmErrR = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
pub type RxfifoOvfR = crate::BitReader;
#[doc = "Field `DSR_CHG` reader - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
pub type DsrChgR = crate::BitReader;
#[doc = "Field `CTS_CHG` reader - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
pub type CtsChgR = crate::BitReader;
#[doc = "Field `BRK_DET` reader - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
pub type BrkDetR = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT` reader - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
pub type RxfifoToutR = crate::BitReader;
#[doc = "Field `SW_XON` reader - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
pub type SwXonR = crate::BitReader;
#[doc = "Field `SW_XOFF` reader - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
pub type SwXoffR = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
pub type GlitchDetR = crate::BitReader;
#[doc = "Field `TX_BRK_DONE` reader - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
pub type TxBrkDoneR = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE` reader - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
pub type TxBrkIdleDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` reader - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `RS485_PARITY_ERR` reader - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
pub type Rs485ParityErrR = crate::BitReader;
#[doc = "Field `RS485_FRM_ERR` reader - This is the status bit for rs485_frm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
pub type Rs485FrmErrR = crate::BitReader;
#[doc = "Field `RS485_CLASH` reader - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
pub type Rs485ClashR = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET` reader - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
pub type AtCmdCharDetR = crate::BitReader;
#[doc = "Field `WAKEUP` reader - This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1."]
pub type WakeupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RxfifoFullR {
        RxfifoFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TxfifoEmptyR {
        TxfifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn parity_err(&self) -> ParityErrR {
        ParityErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn frm_err(&self) -> FrmErrR {
        FrmErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn dsr_chg(&self) -> DsrChgR {
        DsrChgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn cts_chg(&self) -> CtsChgR {
        CtsChgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn brk_det(&self) -> BrkDetR {
        BrkDetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_tout(&self) -> RxfifoToutR {
        RxfifoToutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
    #[inline(always)]
    pub fn sw_xon(&self) -> SwXonR {
        SwXonR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
    #[inline(always)]
    pub fn sw_xoff(&self) -> SwXoffR {
        SwXoffR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GlitchDetR {
        GlitchDetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_brk_done(&self) -> TxBrkDoneR {
        TxBrkDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&self) -> TxBrkIdleDoneR {
        TxBrkIdleDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_parity_err(&self) -> Rs485ParityErrR {
        Rs485ParityErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the status bit for rs485_frm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_frm_err(&self) -> Rs485FrmErrR {
        Rs485FrmErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_clash(&self) -> Rs485ClashR {
        Rs485ClashR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn at_cmd_char_det(&self) -> AtCmdCharDetR {
        AtCmdCharDetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1."]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}

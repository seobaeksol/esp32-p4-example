#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `RXFIFO_FULL` writer - Set this bit to clear the rxfifo_full_int_raw interrupt."]
pub type RxfifoFullW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY` writer - Set this bit to clear txfifo_empty_int_raw interrupt."]
pub type TxfifoEmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PARITY_ERR` writer - Set this bit to clear parity_err_int_raw interrupt."]
pub type ParityErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRM_ERR` writer - Set this bit to clear frm_err_int_raw interrupt."]
pub type FrmErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_OVF` writer - Set this bit to clear rxfifo_ovf_int_raw interrupt."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DSR_CHG` writer - Set this bit to clear the dsr_chg_int_raw interrupt."]
pub type DsrChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CTS_CHG` writer - Set this bit to clear the cts_chg_int_raw interrupt."]
pub type CtsChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BRK_DET` writer - Set this bit to clear the brk_det_int_raw interrupt."]
pub type BrkDetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_TOUT` writer - Set this bit to clear the rxfifo_tout_int_raw interrupt."]
pub type RxfifoToutW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SW_XON` writer - Set this bit to clear the sw_xon_int_raw interrupt."]
pub type SwXonW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SW_XOFF` writer - Set this bit to clear the sw_xoff_int_raw interrupt."]
pub type SwXoffW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GLITCH_DET` writer - Set this bit to clear the glitch_det_int_raw interrupt."]
pub type GlitchDetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_BRK_DONE` writer - Set this bit to clear the tx_brk_done_int_raw interrupt.."]
pub type TxBrkDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_BRK_IDLE_DONE` writer - Set this bit to clear the tx_brk_idle_done_int_raw interrupt."]
pub type TxBrkIdleDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_DONE` writer - Set this bit to clear the tx_done_int_raw interrupt."]
pub type TxDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_PARITY_ERR` writer - Set this bit to clear the rs485_parity_err_int_raw interrupt."]
pub type Rs485ParityErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_FRM_ERR` writer - Set this bit to clear the rs485_frm_err_int_raw interrupt."]
pub type Rs485FrmErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_CLASH` writer - Set this bit to clear the rs485_clash_int_raw interrupt."]
pub type Rs485ClashW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AT_CMD_CHAR_DET` writer - Set this bit to clear the at_cmd_char_det_int_raw interrupt."]
pub type AtCmdCharDetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WAKEUP` writer - Set this bit to clear the uart_wakeup_int_raw interrupt."]
pub type WakeupW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the rxfifo_full_int_raw interrupt."]
    #[inline(always)]
    pub fn rxfifo_full(&mut self) -> RxfifoFullW<'_, IntClrSpec> {
        RxfifoFullW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear txfifo_empty_int_raw interrupt."]
    #[inline(always)]
    pub fn txfifo_empty(&mut self) -> TxfifoEmptyW<'_, IntClrSpec> {
        TxfifoEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear parity_err_int_raw interrupt."]
    #[inline(always)]
    pub fn parity_err(&mut self) -> ParityErrW<'_, IntClrSpec> {
        ParityErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear frm_err_int_raw interrupt."]
    #[inline(always)]
    pub fn frm_err(&mut self) -> FrmErrW<'_, IntClrSpec> {
        FrmErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear rxfifo_ovf_int_raw interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, IntClrSpec> {
        RxfifoOvfW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the dsr_chg_int_raw interrupt."]
    #[inline(always)]
    pub fn dsr_chg(&mut self) -> DsrChgW<'_, IntClrSpec> {
        DsrChgW::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the cts_chg_int_raw interrupt."]
    #[inline(always)]
    pub fn cts_chg(&mut self) -> CtsChgW<'_, IntClrSpec> {
        CtsChgW::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the brk_det_int_raw interrupt."]
    #[inline(always)]
    pub fn brk_det(&mut self) -> BrkDetW<'_, IntClrSpec> {
        BrkDetW::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the rxfifo_tout_int_raw interrupt."]
    #[inline(always)]
    pub fn rxfifo_tout(&mut self) -> RxfifoToutW<'_, IntClrSpec> {
        RxfifoToutW::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the sw_xon_int_raw interrupt."]
    #[inline(always)]
    pub fn sw_xon(&mut self) -> SwXonW<'_, IntClrSpec> {
        SwXonW::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the sw_xoff_int_raw interrupt."]
    #[inline(always)]
    pub fn sw_xoff(&mut self) -> SwXoffW<'_, IntClrSpec> {
        SwXoffW::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear the glitch_det_int_raw interrupt."]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GlitchDetW<'_, IntClrSpec> {
        GlitchDetW::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to clear the tx_brk_done_int_raw interrupt.."]
    #[inline(always)]
    pub fn tx_brk_done(&mut self) -> TxBrkDoneW<'_, IntClrSpec> {
        TxBrkDoneW::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear the tx_brk_idle_done_int_raw interrupt."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&mut self) -> TxBrkIdleDoneW<'_, IntClrSpec> {
        TxBrkIdleDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear the tx_done_int_raw interrupt."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TxDoneW<'_, IntClrSpec> {
        TxDoneW::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear the rs485_parity_err_int_raw interrupt."]
    #[inline(always)]
    pub fn rs485_parity_err(&mut self) -> Rs485ParityErrW<'_, IntClrSpec> {
        Rs485ParityErrW::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to clear the rs485_frm_err_int_raw interrupt."]
    #[inline(always)]
    pub fn rs485_frm_err(&mut self) -> Rs485FrmErrW<'_, IntClrSpec> {
        Rs485FrmErrW::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to clear the rs485_clash_int_raw interrupt."]
    #[inline(always)]
    pub fn rs485_clash(&mut self) -> Rs485ClashW<'_, IntClrSpec> {
        Rs485ClashW::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to clear the at_cmd_char_det_int_raw interrupt."]
    #[inline(always)]
    pub fn at_cmd_char_det(&mut self) -> AtCmdCharDetW<'_, IntClrSpec> {
        AtCmdCharDetW::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to clear the uart_wakeup_int_raw interrupt."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, IntClrSpec> {
        WakeupW::new(self, 19)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}

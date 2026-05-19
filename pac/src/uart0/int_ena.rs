#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `RXFIFO_FULL` reader - This is the enable bit for rxfifo_full_int_st register."]
pub type RxfifoFullR = crate::BitReader;
#[doc = "Field `RXFIFO_FULL` writer - This is the enable bit for rxfifo_full_int_st register."]
pub type RxfifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY` reader - This is the enable bit for txfifo_empty_int_st register."]
pub type TxfifoEmptyR = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` writer - This is the enable bit for txfifo_empty_int_st register."]
pub type TxfifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERR` reader - This is the enable bit for parity_err_int_st register."]
pub type ParityErrR = crate::BitReader;
#[doc = "Field `PARITY_ERR` writer - This is the enable bit for parity_err_int_st register."]
pub type ParityErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRM_ERR` reader - This is the enable bit for frm_err_int_st register."]
pub type FrmErrR = crate::BitReader;
#[doc = "Field `FRM_ERR` writer - This is the enable bit for frm_err_int_st register."]
pub type FrmErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF` reader - This is the enable bit for rxfifo_ovf_int_st register."]
pub type RxfifoOvfR = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` writer - This is the enable bit for rxfifo_ovf_int_st register."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_CHG` reader - This is the enable bit for dsr_chg_int_st register."]
pub type DsrChgR = crate::BitReader;
#[doc = "Field `DSR_CHG` writer - This is the enable bit for dsr_chg_int_st register."]
pub type DsrChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_CHG` reader - This is the enable bit for cts_chg_int_st register."]
pub type CtsChgR = crate::BitReader;
#[doc = "Field `CTS_CHG` writer - This is the enable bit for cts_chg_int_st register."]
pub type CtsChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK_DET` reader - This is the enable bit for brk_det_int_st register."]
pub type BrkDetR = crate::BitReader;
#[doc = "Field `BRK_DET` writer - This is the enable bit for brk_det_int_st register."]
pub type BrkDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_TOUT` reader - This is the enable bit for rxfifo_tout_int_st register."]
pub type RxfifoToutR = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT` writer - This is the enable bit for rxfifo_tout_int_st register."]
pub type RxfifoToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_XON` reader - This is the enable bit for sw_xon_int_st register."]
pub type SwXonR = crate::BitReader;
#[doc = "Field `SW_XON` writer - This is the enable bit for sw_xon_int_st register."]
pub type SwXonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_XOFF` reader - This is the enable bit for sw_xoff_int_st register."]
pub type SwXoffR = crate::BitReader;
#[doc = "Field `SW_XOFF` writer - This is the enable bit for sw_xoff_int_st register."]
pub type SwXoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET` reader - This is the enable bit for glitch_det_int_st register."]
pub type GlitchDetR = crate::BitReader;
#[doc = "Field `GLITCH_DET` writer - This is the enable bit for glitch_det_int_st register."]
pub type GlitchDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BRK_DONE` reader - This is the enable bit for tx_brk_done_int_st register."]
pub type TxBrkDoneR = crate::BitReader;
#[doc = "Field `TX_BRK_DONE` writer - This is the enable bit for tx_brk_done_int_st register."]
pub type TxBrkDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BRK_IDLE_DONE` reader - This is the enable bit for tx_brk_idle_done_int_st register."]
pub type TxBrkIdleDoneR = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE` writer - This is the enable bit for tx_brk_idle_done_int_st register."]
pub type TxBrkIdleDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE` reader - This is the enable bit for tx_done_int_st register."]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` writer - This is the enable bit for tx_done_int_st register."]
pub type TxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485_PARITY_ERR` reader - This is the enable bit for rs485_parity_err_int_st register."]
pub type Rs485ParityErrR = crate::BitReader;
#[doc = "Field `RS485_PARITY_ERR` writer - This is the enable bit for rs485_parity_err_int_st register."]
pub type Rs485ParityErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485_FRM_ERR` reader - This is the enable bit for rs485_parity_err_int_st register."]
pub type Rs485FrmErrR = crate::BitReader;
#[doc = "Field `RS485_FRM_ERR` writer - This is the enable bit for rs485_parity_err_int_st register."]
pub type Rs485FrmErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485_CLASH` reader - This is the enable bit for rs485_clash_int_st register."]
pub type Rs485ClashR = crate::BitReader;
#[doc = "Field `RS485_CLASH` writer - This is the enable bit for rs485_clash_int_st register."]
pub type Rs485ClashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AT_CMD_CHAR_DET` reader - This is the enable bit for at_cmd_char_det_int_st register."]
pub type AtCmdCharDetR = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET` writer - This is the enable bit for at_cmd_char_det_int_st register."]
pub type AtCmdCharDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - This is the enable bit for uart_wakeup_int_st register."]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - This is the enable bit for uart_wakeup_int_st register."]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RxfifoFullR {
        RxfifoFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for txfifo_empty_int_st register."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TxfifoEmptyR {
        TxfifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the enable bit for parity_err_int_st register."]
    #[inline(always)]
    pub fn parity_err(&self) -> ParityErrR {
        ParityErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the enable bit for frm_err_int_st register."]
    #[inline(always)]
    pub fn frm_err(&self) -> FrmErrR {
        FrmErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the enable bit for rxfifo_ovf_int_st register."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the enable bit for dsr_chg_int_st register."]
    #[inline(always)]
    pub fn dsr_chg(&self) -> DsrChgR {
        DsrChgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the enable bit for cts_chg_int_st register."]
    #[inline(always)]
    pub fn cts_chg(&self) -> CtsChgR {
        CtsChgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the enable bit for brk_det_int_st register."]
    #[inline(always)]
    pub fn brk_det(&self) -> BrkDetR {
        BrkDetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the enable bit for rxfifo_tout_int_st register."]
    #[inline(always)]
    pub fn rxfifo_tout(&self) -> RxfifoToutR {
        RxfifoToutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the enable bit for sw_xon_int_st register."]
    #[inline(always)]
    pub fn sw_xon(&self) -> SwXonR {
        SwXonR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the enable bit for sw_xoff_int_st register."]
    #[inline(always)]
    pub fn sw_xoff(&self) -> SwXoffR {
        SwXoffR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for glitch_det_int_st register."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GlitchDetR {
        GlitchDetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for tx_brk_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_done(&self) -> TxBrkDoneR {
        TxBrkDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for tx_brk_idle_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&self) -> TxBrkIdleDoneR {
        TxBrkIdleDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for tx_done_int_st register."]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_parity_err(&self) -> Rs485ParityErrR {
        Rs485ParityErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_frm_err(&self) -> Rs485FrmErrR {
        Rs485FrmErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is the enable bit for rs485_clash_int_st register."]
    #[inline(always)]
    pub fn rs485_clash(&self) -> Rs485ClashR {
        Rs485ClashR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This is the enable bit for at_cmd_char_det_int_st register."]
    #[inline(always)]
    pub fn at_cmd_char_det(&self) -> AtCmdCharDetR {
        AtCmdCharDetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This is the enable bit for uart_wakeup_int_st register."]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn rxfifo_full(&mut self) -> RxfifoFullW<'_, IntEnaSpec> {
        RxfifoFullW::new(self, 0)
    }
    #[doc = "Bit 1 - This is the enable bit for txfifo_empty_int_st register."]
    #[inline(always)]
    pub fn txfifo_empty(&mut self) -> TxfifoEmptyW<'_, IntEnaSpec> {
        TxfifoEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - This is the enable bit for parity_err_int_st register."]
    #[inline(always)]
    pub fn parity_err(&mut self) -> ParityErrW<'_, IntEnaSpec> {
        ParityErrW::new(self, 2)
    }
    #[doc = "Bit 3 - This is the enable bit for frm_err_int_st register."]
    #[inline(always)]
    pub fn frm_err(&mut self) -> FrmErrW<'_, IntEnaSpec> {
        FrmErrW::new(self, 3)
    }
    #[doc = "Bit 4 - This is the enable bit for rxfifo_ovf_int_st register."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, IntEnaSpec> {
        RxfifoOvfW::new(self, 4)
    }
    #[doc = "Bit 5 - This is the enable bit for dsr_chg_int_st register."]
    #[inline(always)]
    pub fn dsr_chg(&mut self) -> DsrChgW<'_, IntEnaSpec> {
        DsrChgW::new(self, 5)
    }
    #[doc = "Bit 6 - This is the enable bit for cts_chg_int_st register."]
    #[inline(always)]
    pub fn cts_chg(&mut self) -> CtsChgW<'_, IntEnaSpec> {
        CtsChgW::new(self, 6)
    }
    #[doc = "Bit 7 - This is the enable bit for brk_det_int_st register."]
    #[inline(always)]
    pub fn brk_det(&mut self) -> BrkDetW<'_, IntEnaSpec> {
        BrkDetW::new(self, 7)
    }
    #[doc = "Bit 8 - This is the enable bit for rxfifo_tout_int_st register."]
    #[inline(always)]
    pub fn rxfifo_tout(&mut self) -> RxfifoToutW<'_, IntEnaSpec> {
        RxfifoToutW::new(self, 8)
    }
    #[doc = "Bit 9 - This is the enable bit for sw_xon_int_st register."]
    #[inline(always)]
    pub fn sw_xon(&mut self) -> SwXonW<'_, IntEnaSpec> {
        SwXonW::new(self, 9)
    }
    #[doc = "Bit 10 - This is the enable bit for sw_xoff_int_st register."]
    #[inline(always)]
    pub fn sw_xoff(&mut self) -> SwXoffW<'_, IntEnaSpec> {
        SwXoffW::new(self, 10)
    }
    #[doc = "Bit 11 - This is the enable bit for glitch_det_int_st register."]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GlitchDetW<'_, IntEnaSpec> {
        GlitchDetW::new(self, 11)
    }
    #[doc = "Bit 12 - This is the enable bit for tx_brk_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_done(&mut self) -> TxBrkDoneW<'_, IntEnaSpec> {
        TxBrkDoneW::new(self, 12)
    }
    #[doc = "Bit 13 - This is the enable bit for tx_brk_idle_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&mut self) -> TxBrkIdleDoneW<'_, IntEnaSpec> {
        TxBrkIdleDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - This is the enable bit for tx_done_int_st register."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TxDoneW<'_, IntEnaSpec> {
        TxDoneW::new(self, 14)
    }
    #[doc = "Bit 15 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_parity_err(&mut self) -> Rs485ParityErrW<'_, IntEnaSpec> {
        Rs485ParityErrW::new(self, 15)
    }
    #[doc = "Bit 16 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_frm_err(&mut self) -> Rs485FrmErrW<'_, IntEnaSpec> {
        Rs485FrmErrW::new(self, 16)
    }
    #[doc = "Bit 17 - This is the enable bit for rs485_clash_int_st register."]
    #[inline(always)]
    pub fn rs485_clash(&mut self) -> Rs485ClashW<'_, IntEnaSpec> {
        Rs485ClashW::new(self, 17)
    }
    #[doc = "Bit 18 - This is the enable bit for at_cmd_char_det_int_st register."]
    #[inline(always)]
    pub fn at_cmd_char_det(&mut self) -> AtCmdCharDetW<'_, IntEnaSpec> {
        AtCmdCharDetW::new(self, 18)
    }
    #[doc = "Bit 19 - This is the enable bit for uart_wakeup_int_st register."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, IntEnaSpec> {
        WakeupW::new(self, 19)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}

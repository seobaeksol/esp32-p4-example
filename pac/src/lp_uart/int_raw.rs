#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `RXFIFO_FULL` reader - This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies."]
pub type RxfifoFullR = crate::BitReader;
#[doc = "Field `RXFIFO_FULL` writer - This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies."]
pub type RxfifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY` reader - This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies ."]
pub type TxfifoEmptyR = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` writer - This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies ."]
pub type TxfifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERR` reader - This interrupt raw bit turns to high level when receiver detects a parity error in the data."]
pub type ParityErrR = crate::BitReader;
#[doc = "Field `PARITY_ERR` writer - This interrupt raw bit turns to high level when receiver detects a parity error in the data."]
pub type ParityErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRM_ERR` reader - This interrupt raw bit turns to high level when receiver detects a data frame error ."]
pub type FrmErrR = crate::BitReader;
#[doc = "Field `FRM_ERR` writer - This interrupt raw bit turns to high level when receiver detects a data frame error ."]
pub type FrmErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF` reader - This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store."]
pub type RxfifoOvfR = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` writer - This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_CHG` reader - This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal."]
pub type DsrChgR = crate::BitReader;
#[doc = "Field `DSR_CHG` writer - This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal."]
pub type DsrChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_CHG` reader - This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal."]
pub type CtsChgR = crate::BitReader;
#[doc = "Field `CTS_CHG` writer - This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal."]
pub type CtsChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK_DET` reader - This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit."]
pub type BrkDetR = crate::BitReader;
#[doc = "Field `BRK_DET` writer - This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit."]
pub type BrkDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_TOUT` reader - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
pub type RxfifoToutR = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT` writer - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
pub type RxfifoToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_XON` reader - This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1."]
pub type SwXonR = crate::BitReader;
#[doc = "Field `SW_XON` writer - This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1."]
pub type SwXonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_XOFF` reader - This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1."]
pub type SwXoffR = crate::BitReader;
#[doc = "Field `SW_XOFF` writer - This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1."]
pub type SwXoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET` reader - This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit."]
pub type GlitchDetR = crate::BitReader;
#[doc = "Field `GLITCH_DET` writer - This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit."]
pub type GlitchDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BRK_DONE` reader - This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent."]
pub type TxBrkDoneR = crate::BitReader;
#[doc = "Field `TX_BRK_DONE` writer - This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent."]
pub type TxBrkDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BRK_IDLE_DONE` reader - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data."]
pub type TxBrkIdleDoneR = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE` writer - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data."]
pub type TxBrkIdleDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE` reader - This interrupt raw bit turns to high level when transmitter has send out all data in FIFO."]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` writer - This interrupt raw bit turns to high level when transmitter has send out all data in FIFO."]
pub type TxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AT_CMD_CHAR_DET` reader - This interrupt raw bit turns to high level when receiver detects the configured at_cmd char."]
pub type AtCmdCharDetR = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET` writer - This interrupt raw bit turns to high level when receiver detects the configured at_cmd char."]
pub type AtCmdCharDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode."]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode."]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RxfifoFullR {
        RxfifoFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies ."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TxfifoEmptyR {
        TxfifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects a parity error in the data."]
    #[inline(always)]
    pub fn parity_err(&self) -> ParityErrR {
        ParityErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects a data frame error ."]
    #[inline(always)]
    pub fn frm_err(&self) -> FrmErrR {
        FrmErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal."]
    #[inline(always)]
    pub fn dsr_chg(&self) -> DsrChgR {
        DsrChgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal."]
    #[inline(always)]
    pub fn cts_chg(&self) -> CtsChgR {
        CtsChgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit."]
    #[inline(always)]
    pub fn brk_det(&self) -> BrkDetR {
        BrkDetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn rxfifo_tout(&self) -> RxfifoToutR {
        RxfifoToutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xon(&self) -> SwXonR {
        SwXonR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xoff(&self) -> SwXoffR {
        SwXoffR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GlitchDetR {
        GlitchDetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent."]
    #[inline(always)]
    pub fn tx_brk_done(&self) -> TxBrkDoneR {
        TxBrkDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&self) -> TxBrkIdleDoneR {
        TxBrkIdleDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send out all data in FIFO."]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd char."]
    #[inline(always)]
    pub fn at_cmd_char_det(&self) -> AtCmdCharDetR {
        AtCmdCharDetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode."]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies."]
    #[inline(always)]
    pub fn rxfifo_full(&mut self) -> RxfifoFullW<'_, IntRawSpec> {
        RxfifoFullW::new(self, 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies ."]
    #[inline(always)]
    pub fn txfifo_empty(&mut self) -> TxfifoEmptyW<'_, IntRawSpec> {
        TxfifoEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects a parity error in the data."]
    #[inline(always)]
    pub fn parity_err(&mut self) -> ParityErrW<'_, IntRawSpec> {
        ParityErrW::new(self, 2)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects a data frame error ."]
    #[inline(always)]
    pub fn frm_err(&mut self) -> FrmErrW<'_, IntRawSpec> {
        FrmErrW::new(self, 3)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, IntRawSpec> {
        RxfifoOvfW::new(self, 4)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal."]
    #[inline(always)]
    pub fn dsr_chg(&mut self) -> DsrChgW<'_, IntRawSpec> {
        DsrChgW::new(self, 5)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal."]
    #[inline(always)]
    pub fn cts_chg(&mut self) -> CtsChgW<'_, IntRawSpec> {
        CtsChgW::new(self, 6)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit."]
    #[inline(always)]
    pub fn brk_det(&mut self) -> BrkDetW<'_, IntRawSpec> {
        BrkDetW::new(self, 7)
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn rxfifo_tout(&mut self) -> RxfifoToutW<'_, IntRawSpec> {
        RxfifoToutW::new(self, 8)
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xon(&mut self) -> SwXonW<'_, IntRawSpec> {
        SwXonW::new(self, 9)
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xoff(&mut self) -> SwXoffW<'_, IntRawSpec> {
        SwXoffW::new(self, 10)
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit."]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GlitchDetW<'_, IntRawSpec> {
        GlitchDetW::new(self, 11)
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sending NULL characters after all data in Tx-FIFO are sent."]
    #[inline(always)]
    pub fn tx_brk_done(&mut self) -> TxBrkDoneW<'_, IntRawSpec> {
        TxBrkDoneW::new(self, 12)
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&mut self) -> TxBrkIdleDoneW<'_, IntRawSpec> {
        TxBrkIdleDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send out all data in FIFO."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TxDoneW<'_, IntRawSpec> {
        TxDoneW::new(self, 14)
    }
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd char."]
    #[inline(always)]
    pub fn at_cmd_char_det(&mut self) -> AtCmdCharDetW<'_, IntRawSpec> {
        AtCmdCharDetW::new(self, 18)
    }
    #[doc = "Bit 19 - This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, IntRawSpec> {
        WakeupW::new(self, 19)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets INT_RAW to value 0x02"]
impl crate::Resettable for IntRawSpec {
    const RESET_VALUE: u32 = 0x02;
}

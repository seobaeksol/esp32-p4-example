#[doc = "Register `CONF0_SYNC` reader"]
pub type R = crate::R<Conf0SyncSpec>;
#[doc = "Register `CONF0_SYNC` writer"]
pub type W = crate::W<Conf0SyncSpec>;
#[doc = "Field `PARITY` reader - This register is used to configure the parity check mode."]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - This register is used to configure the parity check mode."]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_EN` reader - Set this bit to enable uart parity check."]
pub type ParityEnR = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - Set this bit to enable uart parity check."]
pub type ParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NUM` reader - This register is used to set the length of data."]
pub type BitNumR = crate::FieldReader;
#[doc = "Field `BIT_NUM` writer - This register is used to set the length of data."]
pub type BitNumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP_BIT_NUM` reader - This register is used to set the length of stop bit."]
pub type StopBitNumR = crate::FieldReader;
#[doc = "Field `STOP_BIT_NUM` writer - This register is used to set the length of stop bit."]
pub type StopBitNumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXD_BRK` reader - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
pub type TxdBrkR = crate::BitReader;
#[doc = "Field `TXD_BRK` writer - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
pub type TxdBrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Set this bit to enable uart loopback test mode."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Set this bit to enable uart loopback test mode."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FLOW_EN` reader - Set this bit to enable flow control function for transmitter."]
pub type TxFlowEnR = crate::BitReader;
#[doc = "Field `TX_FLOW_EN` writer - Set this bit to enable flow control function for transmitter."]
pub type TxFlowEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXD_INV` reader - Set this bit to inverse the level value of uart rxd signal."]
pub type RxdInvR = crate::BitReader;
#[doc = "Field `RXD_INV` writer - Set this bit to inverse the level value of uart rxd signal."]
pub type RxdInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXD_INV` reader - Set this bit to inverse the level value of uart txd signal."]
pub type TxdInvR = crate::BitReader;
#[doc = "Field `TXD_INV` writer - Set this bit to inverse the level value of uart txd signal."]
pub type TxdInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_RX_DAT_OVF` reader - Disable UART Rx data overflow detect."]
pub type DisRxDatOvfR = crate::BitReader;
#[doc = "Field `DIS_RX_DAT_OVF` writer - Disable UART Rx data overflow detect."]
pub type DisRxDatOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_WR_MASK` reader - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
pub type ErrWrMaskR = crate::BitReader;
#[doc = "Field `ERR_WR_MASK` writer - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
pub type ErrWrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_EN` reader - UART memory clock gate enable signal."]
pub type MemClkEnR = crate::BitReader;
#[doc = "Field `MEM_CLK_EN` writer - UART memory clock gate enable signal."]
pub type MemClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RTS` reader - This register is used to configure the software rts signal which is used in software flow control."]
pub type SwRtsR = crate::BitReader;
#[doc = "Field `SW_RTS` writer - This register is used to configure the software rts signal which is used in software flow control."]
pub type SwRtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_RST` reader - Set this bit to reset the uart receive-FIFO."]
pub type RxfifoRstR = crate::BitReader;
#[doc = "Field `RXFIFO_RST` writer - Set this bit to reset the uart receive-FIFO."]
pub type RxfifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_RST` reader - Set this bit to reset the uart transmit-FIFO."]
pub type TxfifoRstR = crate::BitReader;
#[doc = "Field `TXFIFO_RST` writer - Set this bit to reset the uart transmit-FIFO."]
pub type TxfifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to configure the parity check mode."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable uart parity check."]
    #[inline(always)]
    pub fn parity_en(&self) -> ParityEnR {
        ParityEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This register is used to set the length of data."]
    #[inline(always)]
    pub fn bit_num(&self) -> BitNumR {
        BitNumR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - This register is used to set the length of stop bit."]
    #[inline(always)]
    pub fn stop_bit_num(&self) -> StopBitNumR {
        StopBitNumR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
    #[inline(always)]
    pub fn txd_brk(&self) -> TxdBrkR {
        TxdBrkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable uart loopback test mode."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable flow control function for transmitter."]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TxFlowEnR {
        TxFlowEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to inverse the level value of uart rxd signal."]
    #[inline(always)]
    pub fn rxd_inv(&self) -> RxdInvR {
        RxdInvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to inverse the level value of uart txd signal."]
    #[inline(always)]
    pub fn txd_inv(&self) -> TxdInvR {
        TxdInvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable UART Rx data overflow detect."]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&self) -> DisRxDatOvfR {
        DisRxDatOvfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ErrWrMaskR {
        ErrWrMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - UART memory clock gate enable signal."]
    #[inline(always)]
    pub fn mem_clk_en(&self) -> MemClkEnR {
        MemClkEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This register is used to configure the software rts signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_rts(&self) -> SwRtsR {
        SwRtsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to reset the uart receive-FIFO."]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RxfifoRstR {
        RxfifoRstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to reset the uart transmit-FIFO."]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TxfifoRstR {
        TxfifoRstR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to configure the parity check mode."]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, Conf0SyncSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable uart parity check."]
    #[inline(always)]
    pub fn parity_en(&mut self) -> ParityEnW<'_, Conf0SyncSpec> {
        ParityEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - This register is used to set the length of data."]
    #[inline(always)]
    pub fn bit_num(&mut self) -> BitNumW<'_, Conf0SyncSpec> {
        BitNumW::new(self, 2)
    }
    #[doc = "Bits 4:5 - This register is used to set the length of stop bit."]
    #[inline(always)]
    pub fn stop_bit_num(&mut self) -> StopBitNumW<'_, Conf0SyncSpec> {
        StopBitNumW::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
    #[inline(always)]
    pub fn txd_brk(&mut self) -> TxdBrkW<'_, Conf0SyncSpec> {
        TxdBrkW::new(self, 6)
    }
    #[doc = "Bit 12 - Set this bit to enable uart loopback test mode."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LoopbackW<'_, Conf0SyncSpec> {
        LoopbackW::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to enable flow control function for transmitter."]
    #[inline(always)]
    pub fn tx_flow_en(&mut self) -> TxFlowEnW<'_, Conf0SyncSpec> {
        TxFlowEnW::new(self, 13)
    }
    #[doc = "Bit 15 - Set this bit to inverse the level value of uart rxd signal."]
    #[inline(always)]
    pub fn rxd_inv(&mut self) -> RxdInvW<'_, Conf0SyncSpec> {
        RxdInvW::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to inverse the level value of uart txd signal."]
    #[inline(always)]
    pub fn txd_inv(&mut self) -> TxdInvW<'_, Conf0SyncSpec> {
        TxdInvW::new(self, 16)
    }
    #[doc = "Bit 17 - Disable UART Rx data overflow detect."]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&mut self) -> DisRxDatOvfW<'_, Conf0SyncSpec> {
        DisRxDatOvfW::new(self, 17)
    }
    #[doc = "Bit 18 - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
    #[inline(always)]
    pub fn err_wr_mask(&mut self) -> ErrWrMaskW<'_, Conf0SyncSpec> {
        ErrWrMaskW::new(self, 18)
    }
    #[doc = "Bit 20 - UART memory clock gate enable signal."]
    #[inline(always)]
    pub fn mem_clk_en(&mut self) -> MemClkEnW<'_, Conf0SyncSpec> {
        MemClkEnW::new(self, 20)
    }
    #[doc = "Bit 21 - This register is used to configure the software rts signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_rts(&mut self) -> SwRtsW<'_, Conf0SyncSpec> {
        SwRtsW::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to reset the uart receive-FIFO."]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RxfifoRstW<'_, Conf0SyncSpec> {
        RxfifoRstW::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to reset the uart transmit-FIFO."]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TxfifoRstW<'_, Conf0SyncSpec> {
        TxfifoRstW::new(self, 23)
    }
}
#[doc = "Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conf0SyncSpec;
impl crate::RegisterSpec for Conf0SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0_sync::R`](R) reader structure"]
impl crate::Readable for Conf0SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`conf0_sync::W`](W) writer structure"]
impl crate::Writable for Conf0SyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF0_SYNC to value 0x1c"]
impl crate::Resettable for Conf0SyncSpec {
    const RESET_VALUE: u32 = 0x1c;
}

#[doc = "Register `RX_CTRL` reader"]
pub type R = crate::R<RxCtrlSpec>;
#[doc = "Register `RX_CTRL` writer"]
pub type W = crate::W<RxCtrlSpec>;
#[doc = "Field `RX_ENA` reader - write this bit to enable the bitscrambler rx"]
pub type RxEnaR = crate::BitReader;
#[doc = "Field `RX_ENA` writer - write this bit to enable the bitscrambler rx"]
pub type RxEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PAUSE` reader - write this bit to pause the bitscrambler rx core"]
pub type RxPauseR = crate::BitReader;
#[doc = "Field `RX_PAUSE` writer - write this bit to pause the bitscrambler rx core"]
pub type RxPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HALT` reader - write this bit to halt the bitscrambler rx core"]
pub type RxHaltR = crate::BitReader;
#[doc = "Field `RX_HALT` writer - write this bit to halt the bitscrambler rx core"]
pub type RxHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EOF_MODE` reader - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
pub type RxEofModeR = crate::BitReader;
#[doc = "Field `RX_EOF_MODE` writer - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
pub type RxEofModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_COND_MODE` reader - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
pub type RxCondModeR = crate::BitReader;
#[doc = "Field `RX_COND_MODE` writer - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
pub type RxCondModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FETCH_MODE` reader - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
pub type RxFetchModeR = crate::BitReader;
#[doc = "Field `RX_FETCH_MODE` writer - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
pub type RxFetchModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HALT_MODE` reader - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
pub type RxHaltModeR = crate::BitReader;
#[doc = "Field `RX_HALT_MODE` writer - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
pub type RxHaltModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RD_DUMMY` reader - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
pub type RxRdDummyR = crate::BitReader;
#[doc = "Field `RX_RD_DUMMY` writer - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
pub type RxRdDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RST` writer - write this bit to reset the bitscrambler rx fifo"]
pub type RxFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write this bit to enable the bitscrambler rx"]
    #[inline(always)]
    pub fn rx_ena(&self) -> RxEnaR {
        RxEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write this bit to pause the bitscrambler rx core"]
    #[inline(always)]
    pub fn rx_pause(&self) -> RxPauseR {
        RxPauseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write this bit to halt the bitscrambler rx core"]
    #[inline(always)]
    pub fn rx_halt(&self) -> RxHaltR {
        RxHaltR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
    #[inline(always)]
    pub fn rx_eof_mode(&self) -> RxEofModeR {
        RxEofModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
    #[inline(always)]
    pub fn rx_cond_mode(&self) -> RxCondModeR {
        RxCondModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
    #[inline(always)]
    pub fn rx_fetch_mode(&self) -> RxFetchModeR {
        RxFetchModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
    #[inline(always)]
    pub fn rx_halt_mode(&self) -> RxHaltModeR {
        RxHaltModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
    #[inline(always)]
    pub fn rx_rd_dummy(&self) -> RxRdDummyR {
        RxRdDummyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write this bit to enable the bitscrambler rx"]
    #[inline(always)]
    pub fn rx_ena(&mut self) -> RxEnaW<'_, RxCtrlSpec> {
        RxEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - write this bit to pause the bitscrambler rx core"]
    #[inline(always)]
    pub fn rx_pause(&mut self) -> RxPauseW<'_, RxCtrlSpec> {
        RxPauseW::new(self, 1)
    }
    #[doc = "Bit 2 - write this bit to halt the bitscrambler rx core"]
    #[inline(always)]
    pub fn rx_halt(&mut self) -> RxHaltW<'_, RxCtrlSpec> {
        RxHaltW::new(self, 2)
    }
    #[doc = "Bit 3 - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
    #[inline(always)]
    pub fn rx_eof_mode(&mut self) -> RxEofModeW<'_, RxCtrlSpec> {
        RxEofModeW::new(self, 3)
    }
    #[doc = "Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
    #[inline(always)]
    pub fn rx_cond_mode(&mut self) -> RxCondModeW<'_, RxCtrlSpec> {
        RxCondModeW::new(self, 4)
    }
    #[doc = "Bit 5 - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
    #[inline(always)]
    pub fn rx_fetch_mode(&mut self) -> RxFetchModeW<'_, RxCtrlSpec> {
        RxFetchModeW::new(self, 5)
    }
    #[doc = "Bit 6 - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
    #[inline(always)]
    pub fn rx_halt_mode(&mut self) -> RxHaltModeW<'_, RxCtrlSpec> {
        RxHaltModeW::new(self, 6)
    }
    #[doc = "Bit 7 - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
    #[inline(always)]
    pub fn rx_rd_dummy(&mut self) -> RxRdDummyW<'_, RxCtrlSpec> {
        RxRdDummyW::new(self, 7)
    }
    #[doc = "Bit 8 - write this bit to reset the bitscrambler rx fifo"]
    #[inline(always)]
    pub fn rx_fifo_rst(&mut self) -> RxFifoRstW<'_, RxCtrlSpec> {
        RxFifoRstW::new(self, 8)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCtrlSpec;
impl crate::RegisterSpec for RxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ctrl::R`](R) reader structure"]
impl crate::Readable for RxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_ctrl::W`](W) writer structure"]
impl crate::Writable for RxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CTRL to value 0x04"]
impl crate::Resettable for RxCtrlSpec {
    const RESET_VALUE: u32 = 0x04;
}

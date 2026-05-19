#[doc = "Register `OUT_CONF0` reader"]
pub type R = crate::R<OutConf0Spec>;
#[doc = "Register `OUT_CONF0` writer"]
pub type W = crate::W<OutConf0Spec>;
#[doc = "Field `OUT_RST_CH0` reader - Configures the reset state of AHB_DMA channel 0 TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
pub type OutRstCh0R = crate::BitReader;
#[doc = "Field `OUT_RST_CH0` writer - Configures the reset state of AHB_DMA channel 0 TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
pub type OutRstCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST_CH0` reader - reserved"]
pub type OutLoopTestCh0R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST_CH0` writer - reserved"]
pub type OutLoopTestCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK_CH0` reader - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OutAutoWrbackCh0R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK_CH0` writer - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OutAutoWrbackCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE_CH0` reader - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel 0 is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel 0 is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
pub type OutEofModeCh0R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE_CH0` writer - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel 0 is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel 0 is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
pub type OutEofModeCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN_CH0` reader - Configures whether to enable INCR burst transfer for TX channel 0 reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OutdscrBurstEnCh0R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN_CH0` writer - Configures whether to enable INCR burst transfer for TX channel 0 reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OutdscrBurstEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_EN_CH0` reader - Set this bit to 1 to enable INCR4 burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OutDataBurstEnCh0R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN_CH0` writer - Set this bit to 1 to enable INCR4 burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OutDataBurstEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_EN_CH0` reader - Configures whether to enable ETM control for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OutEtmEnCh0R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN_CH0` writer - Configures whether to enable ETM control for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OutEtmEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_MODE_SEL_CH0` reader - Configures max burst size for TX channel0.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type OutDataBurstModeSelCh0R = crate::FieldReader;
#[doc = "Field `OUT_DATA_BURST_MODE_SEL_CH0` writer - Configures max burst size for TX channel0.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type OutDataBurstModeSelCh0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Configures the reset state of AHB_DMA channel 0 TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
    #[inline(always)]
    pub fn out_rst_ch0(&self) -> OutRstCh0R {
        OutRstCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch0(&self) -> OutLoopTestCh0R {
        OutLoopTestCh0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_auto_wrback_ch0(&self) -> OutAutoWrbackCh0R {
        OutAutoWrbackCh0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel 0 is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel 0 is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
    #[inline(always)]
    pub fn out_eof_mode_ch0(&self) -> OutEofModeCh0R {
        OutEofModeCh0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to enable INCR burst transfer for TX channel 0 reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn outdscr_burst_en_ch0(&self) -> OutdscrBurstEnCh0R {
        OutdscrBurstEnCh0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR4 burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch0(&self) -> OutDataBurstEnCh0R {
        OutDataBurstEnCh0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether to enable ETM control for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_etm_en_ch0(&self) -> OutEtmEnCh0R {
        OutEtmEnCh0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configures max burst size for TX channel0.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn out_data_burst_mode_sel_ch0(&self) -> OutDataBurstModeSelCh0R {
        OutDataBurstModeSelCh0R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the reset state of AHB_DMA channel 0 TX FSM and TX FIFO pointer.\\\\0: Release reset\\\\1: Reset\\\\"]
    #[inline(always)]
    pub fn out_rst_ch0(&mut self) -> OutRstCh0W<'_, OutConf0Spec> {
        OutRstCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch0(&mut self) -> OutLoopTestCh0W<'_, OutConf0Spec> {
        OutLoopTestCh0W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to enable automatic outlink write-back when all the data in TX FIFO has been transmitted.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_auto_wrback_ch0(&mut self) -> OutAutoWrbackCh0W<'_, OutConf0Spec> {
        OutAutoWrbackCh0W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures when to generate EOF flag.\\\\0: EOF flag for TX channel 0 is generated when data to be transmitted has been pushed into FIFO in AHB_DMA.\\\\ 1: EOF flag for TX channel 0 is generated when data to be transmitted has been popped from FIFO in AHB_DMA.\\\\"]
    #[inline(always)]
    pub fn out_eof_mode_ch0(&mut self) -> OutEofModeCh0W<'_, OutConf0Spec> {
        OutEofModeCh0W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether to enable INCR burst transfer for TX channel 0 reading descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn outdscr_burst_en_ch0(&mut self) -> OutdscrBurstEnCh0W<'_, OutConf0Spec> {
        OutdscrBurstEnCh0W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR4 burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch0(&mut self) -> OutDataBurstEnCh0W<'_, OutConf0Spec> {
        OutDataBurstEnCh0W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether to enable ETM control for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_etm_en_ch0(&mut self) -> OutEtmEnCh0W<'_, OutConf0Spec> {
        OutEtmEnCh0W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configures max burst size for TX channel0.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn out_data_burst_mode_sel_ch0(&mut self) -> OutDataBurstModeSelCh0W<'_, OutConf0Spec> {
        OutDataBurstModeSelCh0W::new(self, 8)
    }
}
#[doc = "Configuration register 0 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutConf0Spec;
impl crate::RegisterSpec for OutConf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0::R`](R) reader structure"]
impl crate::Readable for OutConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`out_conf0::W`](W) writer structure"]
impl crate::Writable for OutConf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CONF0 to value 0x0108"]
impl crate::Resettable for OutConf0Spec {
    const RESET_VALUE: u32 = 0x0108;
}

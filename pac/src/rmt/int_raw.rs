#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `CH0_TX_END` reader - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
pub type Ch0TxEndR = crate::BitReader;
#[doc = "Field `CH0_TX_END` writer - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
pub type Ch0TxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_END` reader - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
pub type Ch1TxEndR = crate::BitReader;
#[doc = "Field `CH1_TX_END` writer - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
pub type Ch1TxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_END` reader - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
pub type Ch2TxEndR = crate::BitReader;
#[doc = "Field `CH2_TX_END` writer - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
pub type Ch2TxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_END` reader - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
pub type Ch3TxEndR = crate::BitReader;
#[doc = "Field `CH3_TX_END` writer - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
pub type Ch3TxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_ERR` reader - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
pub type Ch0ErrR = crate::BitReader;
#[doc = "Field `CH0_ERR` writer - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
pub type Ch0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ERR` reader - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
pub type Ch1ErrR = crate::BitReader;
#[doc = "Field `CH1_ERR` writer - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
pub type Ch1ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_ERR` reader - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
pub type Ch2ErrR = crate::BitReader;
#[doc = "Field `CH2_ERR` writer - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
pub type Ch2ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_ERR` reader - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
pub type Ch3ErrR = crate::BitReader;
#[doc = "Field `CH3_ERR` writer - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
pub type Ch3ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_THR_EVENT` reader - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
pub type Ch0TxThrEventR = crate::BitReader;
#[doc = "Field `CH0_TX_THR_EVENT` writer - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
pub type Ch0TxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_THR_EVENT` reader - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
pub type Ch1TxThrEventR = crate::BitReader;
#[doc = "Field `CH1_TX_THR_EVENT` writer - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
pub type Ch1TxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_THR_EVENT` reader - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
pub type Ch2TxThrEventR = crate::BitReader;
#[doc = "Field `CH2_TX_THR_EVENT` writer - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
pub type Ch2TxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_THR_EVENT` reader - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
pub type Ch3TxThrEventR = crate::BitReader;
#[doc = "Field `CH3_TX_THR_EVENT` writer - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
pub type Ch3TxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_LOOP` reader - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
pub type Ch0TxLoopR = crate::BitReader;
#[doc = "Field `CH0_TX_LOOP` writer - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
pub type Ch0TxLoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_LOOP` reader - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
pub type Ch1TxLoopR = crate::BitReader;
#[doc = "Field `CH1_TX_LOOP` writer - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
pub type Ch1TxLoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_LOOP` reader - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
pub type Ch2TxLoopR = crate::BitReader;
#[doc = "Field `CH2_TX_LOOP` writer - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
pub type Ch2TxLoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_LOOP` reader - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
pub type Ch3TxLoopR = crate::BitReader;
#[doc = "Field `CH3_TX_LOOP` writer - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
pub type Ch3TxLoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_END` reader - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type Ch4RxEndR = crate::BitReader;
#[doc = "Field `CH4_RX_END` writer - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type Ch4RxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_END` reader - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
pub type Ch5RxEndR = crate::BitReader;
#[doc = "Field `CH5_RX_END` writer - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
pub type Ch5RxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_END` reader - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
pub type Ch6RxEndR = crate::BitReader;
#[doc = "Field `CH6_RX_END` writer - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
pub type Ch6RxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_END` reader - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
pub type Ch7RxEndR = crate::BitReader;
#[doc = "Field `CH7_RX_END` writer - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
pub type Ch7RxEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_ERR` reader - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type Ch4ErrR = crate::BitReader;
#[doc = "Field `CH4_ERR` writer - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type Ch4ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_ERR` reader - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
pub type Ch5ErrR = crate::BitReader;
#[doc = "Field `CH5_ERR` writer - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
pub type Ch5ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_ERR` reader - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type Ch6ErrR = crate::BitReader;
#[doc = "Field `CH6_ERR` writer - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type Ch6ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_ERR` reader - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
pub type Ch7ErrR = crate::BitReader;
#[doc = "Field `CH7_ERR` writer - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
pub type Ch7ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_THR_EVENT` reader - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type Ch4RxThrEventR = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT` writer - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type Ch4RxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_THR_EVENT` reader - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
pub type Ch5RxThrEventR = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT` writer - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
pub type Ch5RxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_THR_EVENT` reader - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
pub type Ch6RxThrEventR = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT` writer - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
pub type Ch6RxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_THR_EVENT` reader - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
pub type Ch7RxThrEventR = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT` writer - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
pub type Ch7RxThrEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_DMA_ACCESS_FAIL` reader - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type Ch3DmaAccessFailR = crate::BitReader;
#[doc = "Field `CH3_DMA_ACCESS_FAIL` writer - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type Ch3DmaAccessFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_DMA_ACCESS_FAIL` reader - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type Ch7DmaAccessFailR = crate::BitReader;
#[doc = "Field `CH7_DMA_ACCESS_FAIL` writer - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type Ch7DmaAccessFailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> Ch0TxEndR {
        Ch0TxEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> Ch1TxEndR {
        Ch1TxEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> Ch2TxEndR {
        Ch2TxEndR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> Ch3TxEndR {
        Ch3TxEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_err(&self) -> Ch0ErrR {
        Ch0ErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_err(&self) -> Ch1ErrR {
        Ch1ErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_err(&self) -> Ch2ErrR {
        Ch2ErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_err(&self) -> Ch3ErrR {
        Ch3ErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> Ch0TxThrEventR {
        Ch0TxThrEventR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> Ch1TxThrEventR {
        Ch1TxThrEventR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> Ch2TxThrEventR {
        Ch2TxThrEventR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> Ch3TxThrEventR {
        Ch3TxThrEventR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> Ch0TxLoopR {
        Ch0TxLoopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> Ch1TxLoopR {
        Ch1TxLoopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> Ch2TxLoopR {
        Ch2TxLoopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> Ch3TxLoopR {
        Ch3TxLoopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> Ch4RxEndR {
        Ch4RxEndR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> Ch5RxEndR {
        Ch5RxEndR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> Ch6RxEndR {
        Ch6RxEndR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> Ch7RxEndR {
        Ch7RxEndR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch4_err(&self) -> Ch4ErrR {
        Ch4ErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch5_err(&self) -> Ch5ErrR {
        Ch5ErrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch6_err(&self) -> Ch6ErrR {
        Ch6ErrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch7_err(&self) -> Ch7ErrR {
        Ch7ErrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&self) -> Ch4RxThrEventR {
        Ch4RxThrEventR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&self) -> Ch5RxThrEventR {
        Ch5RxThrEventR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&self) -> Ch6RxThrEventR {
        Ch6RxThrEventR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&self) -> Ch7RxThrEventR {
        Ch7RxThrEventR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    pub fn ch3_dma_access_fail(&self) -> Ch3DmaAccessFailR {
        Ch3DmaAccessFailR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    pub fn ch7_dma_access_fail(&self) -> Ch7DmaAccessFailR {
        Ch7DmaAccessFailR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end(&mut self) -> Ch0TxEndW<'_, IntRawSpec> {
        Ch0TxEndW::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end(&mut self) -> Ch1TxEndW<'_, IntRawSpec> {
        Ch1TxEndW::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch2_tx_end(&mut self) -> Ch2TxEndW<'_, IntRawSpec> {
        Ch2TxEndW::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch3_tx_end(&mut self) -> Ch3TxEndW<'_, IntRawSpec> {
        Ch3TxEndW::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_err(&mut self) -> Ch0ErrW<'_, IntRawSpec> {
        Ch0ErrW::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_err(&mut self) -> Ch1ErrW<'_, IntRawSpec> {
        Ch1ErrW::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_err(&mut self) -> Ch2ErrW<'_, IntRawSpec> {
        Ch2ErrW::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_err(&mut self) -> Ch3ErrW<'_, IntRawSpec> {
        Ch3ErrW::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&mut self) -> Ch0TxThrEventW<'_, IntRawSpec> {
        Ch0TxThrEventW::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&mut self) -> Ch1TxThrEventW<'_, IntRawSpec> {
        Ch1TxThrEventW::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&mut self) -> Ch2TxThrEventW<'_, IntRawSpec> {
        Ch2TxThrEventW::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&mut self) -> Ch3TxThrEventW<'_, IntRawSpec> {
        Ch3TxThrEventW::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop(&mut self) -> Ch0TxLoopW<'_, IntRawSpec> {
        Ch0TxLoopW::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop(&mut self) -> Ch1TxLoopW<'_, IntRawSpec> {
        Ch1TxLoopW::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch2_tx_loop(&mut self) -> Ch2TxLoopW<'_, IntRawSpec> {
        Ch2TxLoopW::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch3_tx_loop(&mut self) -> Ch3TxLoopW<'_, IntRawSpec> {
        Ch3TxLoopW::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch4_rx_end(&mut self) -> Ch4RxEndW<'_, IntRawSpec> {
        Ch4RxEndW::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
    #[inline(always)]
    pub fn ch5_rx_end(&mut self) -> Ch5RxEndW<'_, IntRawSpec> {
        Ch5RxEndW::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
    #[inline(always)]
    pub fn ch6_rx_end(&mut self) -> Ch6RxEndW<'_, IntRawSpec> {
        Ch6RxEndW::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
    #[inline(always)]
    pub fn ch7_rx_end(&mut self) -> Ch7RxEndW<'_, IntRawSpec> {
        Ch7RxEndW::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch4_err(&mut self) -> Ch4ErrW<'_, IntRawSpec> {
        Ch4ErrW::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch5_err(&mut self) -> Ch5ErrW<'_, IntRawSpec> {
        Ch5ErrW::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch6_err(&mut self) -> Ch6ErrW<'_, IntRawSpec> {
        Ch6ErrW::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch7_err(&mut self) -> Ch7ErrW<'_, IntRawSpec> {
        Ch7ErrW::new(self, 23)
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&mut self) -> Ch4RxThrEventW<'_, IntRawSpec> {
        Ch4RxThrEventW::new(self, 24)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&mut self) -> Ch5RxThrEventW<'_, IntRawSpec> {
        Ch5RxThrEventW::new(self, 25)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&mut self) -> Ch6RxThrEventW<'_, IntRawSpec> {
        Ch6RxThrEventW::new(self, 26)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&mut self) -> Ch7RxThrEventW<'_, IntRawSpec> {
        Ch7RxThrEventW::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    pub fn ch3_dma_access_fail(&mut self) -> Ch3DmaAccessFailW<'_, IntRawSpec> {
        Ch3DmaAccessFailW::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    pub fn ch7_dma_access_fail(&mut self) -> Ch7DmaAccessFailW<'_, IntRawSpec> {
        Ch7DmaAccessFailW::new(self, 29)
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
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}

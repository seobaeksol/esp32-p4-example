#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `CH0_TX_END` reader - The masked interrupt status bit for CH0_TX_END_INT."]
pub type Ch0TxEndR = crate::BitReader;
#[doc = "Field `CH1_TX_END` reader - The masked interrupt status bit for CH1_TX_END_INT."]
pub type Ch1TxEndR = crate::BitReader;
#[doc = "Field `CH2_TX_END` reader - The masked interrupt status bit for CH2_TX_END_INT."]
pub type Ch2TxEndR = crate::BitReader;
#[doc = "Field `CH3_TX_END` reader - The masked interrupt status bit for CH3_TX_END_INT."]
pub type Ch3TxEndR = crate::BitReader;
#[doc = "Field `CH0_ERR` reader - The masked interrupt status bit for CH0_ERR_INT."]
pub type Ch0ErrR = crate::BitReader;
#[doc = "Field `CH1_ERR` reader - The masked interrupt status bit for CH1_ERR_INT."]
pub type Ch1ErrR = crate::BitReader;
#[doc = "Field `CH2_ERR` reader - The masked interrupt status bit for CH2_ERR_INT."]
pub type Ch2ErrR = crate::BitReader;
#[doc = "Field `CH3_ERR` reader - The masked interrupt status bit for CH3_ERR_INT."]
pub type Ch3ErrR = crate::BitReader;
#[doc = "Field `CH0_TX_THR_EVENT` reader - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
pub type Ch0TxThrEventR = crate::BitReader;
#[doc = "Field `CH1_TX_THR_EVENT` reader - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
pub type Ch1TxThrEventR = crate::BitReader;
#[doc = "Field `CH2_TX_THR_EVENT` reader - The masked interrupt status bit for CH2_TX_THR_EVENT_INT."]
pub type Ch2TxThrEventR = crate::BitReader;
#[doc = "Field `CH3_TX_THR_EVENT` reader - The masked interrupt status bit for CH3_TX_THR_EVENT_INT."]
pub type Ch3TxThrEventR = crate::BitReader;
#[doc = "Field `CH0_TX_LOOP` reader - The masked interrupt status bit for CH0_TX_LOOP_INT."]
pub type Ch0TxLoopR = crate::BitReader;
#[doc = "Field `CH1_TX_LOOP` reader - The masked interrupt status bit for CH1_TX_LOOP_INT."]
pub type Ch1TxLoopR = crate::BitReader;
#[doc = "Field `CH2_TX_LOOP` reader - The masked interrupt status bit for CH2_TX_LOOP_INT."]
pub type Ch2TxLoopR = crate::BitReader;
#[doc = "Field `CH3_TX_LOOP` reader - The masked interrupt status bit for CH3_TX_LOOP_INT."]
pub type Ch3TxLoopR = crate::BitReader;
#[doc = "Field `CH4_RX_END` reader - The masked interrupt status bit for CH4_RX_END_INT."]
pub type Ch4RxEndR = crate::BitReader;
#[doc = "Field `CH5_RX_END` reader - The masked interrupt status bit for CH5_RX_END_INT."]
pub type Ch5RxEndR = crate::BitReader;
#[doc = "Field `CH6_RX_END` reader - The masked interrupt status bit for CH6_RX_END_INT."]
pub type Ch6RxEndR = crate::BitReader;
#[doc = "Field `CH7_RX_END` reader - The masked interrupt status bit for CH7_RX_END_INT."]
pub type Ch7RxEndR = crate::BitReader;
#[doc = "Field `CH4_ERR` reader - The masked interrupt status bit for CH4_ERR_INT."]
pub type Ch4ErrR = crate::BitReader;
#[doc = "Field `CH5_ERR` reader - The masked interrupt status bit for CH5_ERR_INT."]
pub type Ch5ErrR = crate::BitReader;
#[doc = "Field `CH6_ERR` reader - The masked interrupt status bit for CH6_ERR_INT."]
pub type Ch6ErrR = crate::BitReader;
#[doc = "Field `CH7_ERR` reader - The masked interrupt status bit for CH7_ERR_INT."]
pub type Ch7ErrR = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT` reader - The masked interrupt status bit for CH4_RX_THR_EVENT_INT."]
pub type Ch4RxThrEventR = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT` reader - The masked interrupt status bit for CH5_RX_THR_EVENT_INT."]
pub type Ch5RxThrEventR = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT` reader - The masked interrupt status bit for CH6_RX_THR_EVENT_INT."]
pub type Ch6RxThrEventR = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT` reader - The masked interrupt status bit for CH7_RX_THR_EVENT_INT."]
pub type Ch7RxThrEventR = crate::BitReader;
#[doc = "Field `CH3_DMA_ACCESS_FAIL` reader - The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type Ch3DmaAccessFailR = crate::BitReader;
#[doc = "Field `CH7_DMA_ACCESS_FAIL` reader - The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type Ch7DmaAccessFailR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> Ch0TxEndR {
        Ch0TxEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> Ch1TxEndR {
        Ch1TxEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> Ch2TxEndR {
        Ch2TxEndR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> Ch3TxEndR {
        Ch3TxEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err(&self) -> Ch0ErrR {
        Ch0ErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err(&self) -> Ch1ErrR {
        Ch1ErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err(&self) -> Ch2ErrR {
        Ch2ErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err(&self) -> Ch3ErrR {
        Ch3ErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> Ch0TxThrEventR {
        Ch0TxThrEventR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> Ch1TxThrEventR {
        Ch1TxThrEventR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> Ch2TxThrEventR {
        Ch2TxThrEventR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> Ch3TxThrEventR {
        Ch3TxThrEventR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> Ch0TxLoopR {
        Ch0TxLoopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> Ch1TxLoopR {
        Ch1TxLoopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> Ch2TxLoopR {
        Ch2TxLoopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> Ch3TxLoopR {
        Ch3TxLoopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The masked interrupt status bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> Ch4RxEndR {
        Ch4RxEndR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The masked interrupt status bit for CH5_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> Ch5RxEndR {
        Ch5RxEndR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The masked interrupt status bit for CH6_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> Ch6RxEndR {
        Ch6RxEndR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The masked interrupt status bit for CH7_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> Ch7RxEndR {
        Ch7RxEndR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The masked interrupt status bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_err(&self) -> Ch4ErrR {
        Ch4ErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The masked interrupt status bit for CH5_ERR_INT."]
    #[inline(always)]
    pub fn ch5_err(&self) -> Ch5ErrR {
        Ch5ErrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The masked interrupt status bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch6_err(&self) -> Ch6ErrR {
        Ch6ErrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The masked interrupt status bit for CH7_ERR_INT."]
    #[inline(always)]
    pub fn ch7_err(&self) -> Ch7ErrR {
        Ch7ErrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The masked interrupt status bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&self) -> Ch4RxThrEventR {
        Ch4RxThrEventR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The masked interrupt status bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&self) -> Ch5RxThrEventR {
        Ch5RxThrEventR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The masked interrupt status bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&self) -> Ch6RxThrEventR {
        Ch6RxThrEventR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The masked interrupt status bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&self) -> Ch7RxThrEventR {
        Ch7RxThrEventR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn ch3_dma_access_fail(&self) -> Ch3DmaAccessFailR {
        Ch3DmaAccessFailR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn ch7_dma_access_fail(&self) -> Ch7DmaAccessFailR {
        Ch7DmaAccessFailR::new(((self.bits >> 29) & 1) != 0)
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

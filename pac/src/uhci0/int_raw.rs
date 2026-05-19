#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `RX_START` reader - Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully."]
pub type RxStartR = crate::BitReader;
#[doc = "Field `RX_START` writer - Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully."]
pub type RxStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START` reader - Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter."]
pub type TxStartR = crate::BitReader;
#[doc = "Field `TX_START` writer - Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter."]
pub type TxStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG` reader - Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value."]
pub type RxHungR = crate::BitReader;
#[doc = "Field `RX_HUNG` writer - Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value."]
pub type RxHungW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG` reader - Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value."]
pub type TxHungR = crate::BitReader;
#[doc = "Field `TX_HUNG` writer - Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value."]
pub type TxHungW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q` reader - Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode."]
pub type SendSRegQR = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q` writer - Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode."]
pub type SendSRegQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q` reader - Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode."]
pub type SendARegQR = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q` writer - Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode."]
pub type SendARegQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF."]
pub type OutEofR = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF."]
pub type OutEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0` reader - Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1."]
pub type AppCtrl0R = crate::BitReader;
#[doc = "Field `APP_CTRL0` writer - Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1."]
pub type AppCtrl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1` reader - Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1."]
pub type AppCtrl1R = crate::BitReader;
#[doc = "Field `APP_CTRL1` writer - Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1."]
pub type AppCtrl1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully."]
    #[inline(always)]
    pub fn rx_start(&self) -> RxStartR {
        RxStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter."]
    #[inline(always)]
    pub fn tx_start(&self) -> TxStartR {
        TxStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value."]
    #[inline(always)]
    pub fn rx_hung(&self) -> RxHungR {
        RxHungR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value."]
    #[inline(always)]
    pub fn tx_hung(&self) -> TxHungR {
        TxHungR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode."]
    #[inline(always)]
    pub fn send_s_reg_q(&self) -> SendSRegQR {
        SendSRegQR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode."]
    #[inline(always)]
    pub fn send_a_reg_q(&self) -> SendARegQR {
        SendARegQR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF."]
    #[inline(always)]
    pub fn out_eof(&self) -> OutEofR {
        OutEofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1."]
    #[inline(always)]
    pub fn app_ctrl0(&self) -> AppCtrl0R {
        AppCtrl0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1."]
    #[inline(always)]
    pub fn app_ctrl1(&self) -> AppCtrl1R {
        AppCtrl1R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the raw interrupt of UHCI_RX_START_INT. Interrupt will be triggered when delimiter is sent successfully."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RxStartW<'_, IntRawSpec> {
        RxStartW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates the raw interrupt of UHCI_TX_START_INT. Interrupt will be triggered when DMA detects delimiter."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TxStartW<'_, IntRawSpec> {
        TxStartW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates the raw interrupt of UHCI_RX_HUNG_INT. Interrupt will be triggered when the required time of DMA receiving data exceeds the configuration value."]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RxHungW<'_, IntRawSpec> {
        RxHungW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates the raw interrupt of UHCI_TX_HUNG_INT. Interrupt will be triggered when the required time of DMA reading RAM data exceeds the configuration value."]
    #[inline(always)]
    pub fn tx_hung(&mut self) -> TxHungW<'_, IntRawSpec> {
        TxHungW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates the raw interrupt of UHCI_SEND_S_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with single_send mode."]
    #[inline(always)]
    pub fn send_s_reg_q(&mut self) -> SendSRegQW<'_, IntRawSpec> {
        SendSRegQW::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates the raw interrupt of UHCI_SEND_A_REG_Q_INT. Interrupt will be triggered when UHCI sends short packet successfully with always_send mode."]
    #[inline(always)]
    pub fn send_a_reg_q(&mut self) -> SendARegQW<'_, IntRawSpec> {
        SendARegQW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates the raw interrupt of UHCI_OUT_EOF_INT. Interrupt will be triggered when there are errors in EOF."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OutEofW<'_, IntRawSpec> {
        OutEofW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates the raw interrupt of UHCI_APP_CTRL0_INT. Interrupt will be triggered when UHCI_APP_CTRL0_IN_SET is set to 1."]
    #[inline(always)]
    pub fn app_ctrl0(&mut self) -> AppCtrl0W<'_, IntRawSpec> {
        AppCtrl0W::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates the raw interrupt of UHCI_APP_CTRL1_INT. Interrupt will be triggered when UHCI_APP_CTRL1_IN_SET is set to 1."]
    #[inline(always)]
    pub fn app_ctrl1(&mut self) -> AppCtrl1W<'_, IntRawSpec> {
        AppCtrl1W::new(self, 8)
    }
}
#[doc = "UHCI Interrupt Raw Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

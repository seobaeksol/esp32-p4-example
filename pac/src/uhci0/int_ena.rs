#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `RX_START` reader - Set this bit to enable the interrupt of UHCI_RX_START_INT."]
pub type RxStartR = crate::BitReader;
#[doc = "Field `RX_START` writer - Set this bit to enable the interrupt of UHCI_RX_START_INT."]
pub type RxStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START` reader - Set this bit to enable the interrupt of UHCI_TX_START_INT."]
pub type TxStartR = crate::BitReader;
#[doc = "Field `TX_START` writer - Set this bit to enable the interrupt of UHCI_TX_START_INT."]
pub type TxStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG` reader - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT."]
pub type RxHungR = crate::BitReader;
#[doc = "Field `RX_HUNG` writer - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT."]
pub type RxHungW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG` reader - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT."]
pub type TxHungR = crate::BitReader;
#[doc = "Field `TX_HUNG` writer - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT."]
pub type TxHungW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q` reader - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT."]
pub type SendSRegQR = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q` writer - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT."]
pub type SendSRegQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q` reader - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT."]
pub type SendARegQR = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q` writer - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT."]
pub type SendARegQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_EOF_ERR` reader - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT."]
pub type OutlinkEofErrR = crate::BitReader;
#[doc = "Field `OUTLINK_EOF_ERR` writer - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT."]
pub type OutlinkEofErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0` reader - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT."]
pub type AppCtrl0R = crate::BitReader;
#[doc = "Field `APP_CTRL0` writer - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT."]
pub type AppCtrl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1` reader - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT."]
pub type AppCtrl1R = crate::BitReader;
#[doc = "Field `APP_CTRL1` writer - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT."]
pub type AppCtrl1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable the interrupt of UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start(&self) -> RxStartR {
        RxStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable the interrupt of UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start(&self) -> TxStartR {
        TxStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung(&self) -> RxHungR {
        RxHungR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung(&self) -> TxHungR {
        TxHungR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q(&self) -> SendSRegQR {
        SendSRegQR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q(&self) -> SendARegQR {
        SendARegQR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT."]
    #[inline(always)]
    pub fn outlink_eof_err(&self) -> OutlinkEofErrR {
        OutlinkEofErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0(&self) -> AppCtrl0R {
        AppCtrl0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1(&self) -> AppCtrl1R {
        AppCtrl1R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable the interrupt of UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RxStartW<'_, IntEnaSpec> {
        RxStartW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable the interrupt of UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TxStartW<'_, IntEnaSpec> {
        TxStartW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RxHungW<'_, IntEnaSpec> {
        RxHungW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung(&mut self) -> TxHungW<'_, IntEnaSpec> {
        TxHungW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q(&mut self) -> SendSRegQW<'_, IntEnaSpec> {
        SendSRegQW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q(&mut self) -> SendARegQW<'_, IntEnaSpec> {
        SendARegQW::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT."]
    #[inline(always)]
    pub fn outlink_eof_err(&mut self) -> OutlinkEofErrW<'_, IntEnaSpec> {
        OutlinkEofErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0(&mut self) -> AppCtrl0W<'_, IntEnaSpec> {
        AppCtrl0W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1(&mut self) -> AppCtrl1W<'_, IntEnaSpec> {
        AppCtrl1W::new(self, 8)
    }
}
#[doc = "UHCI Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

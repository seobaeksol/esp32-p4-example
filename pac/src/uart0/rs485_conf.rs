#[doc = "Register `RS485_CONF` reader"]
pub type R = crate::R<Rs485ConfSpec>;
#[doc = "Register `RS485_CONF` writer"]
pub type W = crate::W<Rs485ConfSpec>;
#[doc = "Field `RS485_EN` reader - Set this bit to choose the rs485 mode."]
pub type Rs485EnR = crate::BitReader;
#[doc = "Field `RS485_EN` writer - Set this bit to choose the rs485 mode."]
pub type Rs485EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type Dl0EnR = crate::BitReader;
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type Dl0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type Dl1EnR = crate::BitReader;
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type Dl1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485TX_RX_EN` reader - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
pub type Rs485txRxEnR = crate::BitReader;
#[doc = "Field `RS485TX_RX_EN` writer - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
pub type Rs485txRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485RXBY_TX_EN` reader - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
pub type Rs485rxbyTxEnR = crate::BitReader;
#[doc = "Field `RS485RXBY_TX_EN` writer - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
pub type Rs485rxbyTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485_RX_DLY_NUM` reader - This register is used to delay the receiver's internal data signal."]
pub type Rs485RxDlyNumR = crate::BitReader;
#[doc = "Field `RS485_RX_DLY_NUM` writer - This register is used to delay the receiver's internal data signal."]
pub type Rs485RxDlyNumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485_TX_DLY_NUM` reader - This register is used to delay the transmitter's internal data signal."]
pub type Rs485TxDlyNumR = crate::FieldReader;
#[doc = "Field `RS485_TX_DLY_NUM` writer - This register is used to delay the transmitter's internal data signal."]
pub type Rs485TxDlyNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Set this bit to choose the rs485 mode."]
    #[inline(always)]
    pub fn rs485_en(&self) -> Rs485EnR {
        Rs485EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&self) -> Dl0EnR {
        Dl0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&self) -> Dl1EnR {
        Dl1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&self) -> Rs485txRxEnR {
        Rs485txRxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
    #[inline(always)]
    pub fn rs485rxby_tx_en(&self) -> Rs485rxbyTxEnR {
        Rs485rxbyTxEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn rs485_rx_dly_num(&self) -> Rs485RxDlyNumR {
        Rs485RxDlyNumR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn rs485_tx_dly_num(&self) -> Rs485TxDlyNumR {
        Rs485TxDlyNumR::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to choose the rs485 mode."]
    #[inline(always)]
    pub fn rs485_en(&mut self) -> Rs485EnW<'_, Rs485ConfSpec> {
        Rs485EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&mut self) -> Dl0EnW<'_, Rs485ConfSpec> {
        Dl0EnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&mut self) -> Dl1EnW<'_, Rs485ConfSpec> {
        Dl1EnW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&mut self) -> Rs485txRxEnW<'_, Rs485ConfSpec> {
        Rs485txRxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
    #[inline(always)]
    pub fn rs485rxby_tx_en(&mut self) -> Rs485rxbyTxEnW<'_, Rs485ConfSpec> {
        Rs485rxbyTxEnW::new(self, 4)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn rs485_rx_dly_num(&mut self) -> Rs485RxDlyNumW<'_, Rs485ConfSpec> {
        Rs485RxDlyNumW::new(self, 5)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn rs485_tx_dly_num(&mut self) -> Rs485TxDlyNumW<'_, Rs485ConfSpec> {
        Rs485TxDlyNumW::new(self, 6)
    }
}
#[doc = "RS485 mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs485ConfSpec;
impl crate::RegisterSpec for Rs485ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485_conf::R`](R) reader structure"]
impl crate::Readable for Rs485ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`rs485_conf::W`](W) writer structure"]
impl crate::Writable for Rs485ConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RS485_CONF to value 0"]
impl crate::Resettable for Rs485ConfSpec {}

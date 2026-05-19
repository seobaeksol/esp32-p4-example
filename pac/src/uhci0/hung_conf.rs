#[doc = "Register `HUNG_CONF` reader"]
pub type R = crate::R<HungConfSpec>;
#[doc = "Register `HUNG_CONF` writer"]
pub type W = crate::W<HungConfSpec>;
#[doc = "Field `TXFIFO_TIMEOUT` reader - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
pub type TxfifoTimeoutR = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT` writer - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
pub type TxfifoTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` reader - Configures the maximum counter value."]
pub type TxfifoTimeoutShiftR = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` writer - Configures the maximum counter value."]
pub type TxfifoTimeoutShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` reader - Set this bit to enable TX FIFO timeout when receiving."]
pub type TxfifoTimeoutEnaR = crate::BitReader;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` writer - Set this bit to enable TX FIFO timeout when receiving."]
pub type TxfifoTimeoutEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_TIMEOUT` reader - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
pub type RxfifoTimeoutR = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT` writer - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
pub type RxfifoTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` reader - Configures the maximum counter value."]
pub type RxfifoTimeoutShiftR = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` writer - Configures the maximum counter value."]
pub type RxfifoTimeoutShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` reader - Set this bit to enable TX FIFO timeout when DMA sending data."]
pub type RxfifoTimeoutEnaR = crate::BitReader;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` writer - Set this bit to enable TX FIFO timeout when DMA sending data."]
pub type RxfifoTimeoutEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
    #[inline(always)]
    pub fn txfifo_timeout(&self) -> TxfifoTimeoutR {
        TxfifoTimeoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Configures the maximum counter value."]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&self) -> TxfifoTimeoutShiftR {
        TxfifoTimeoutShiftR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Set this bit to enable TX FIFO timeout when receiving."]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&self) -> TxfifoTimeoutEnaR {
        TxfifoTimeoutEnaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
    #[inline(always)]
    pub fn rxfifo_timeout(&self) -> RxfifoTimeoutR {
        RxfifoTimeoutR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:22 - Configures the maximum counter value."]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&self) -> RxfifoTimeoutShiftR {
        RxfifoTimeoutShiftR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Set this bit to enable TX FIFO timeout when DMA sending data."]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&self) -> RxfifoTimeoutEnaR {
        RxfifoTimeoutEnaR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when receiving data."]
    #[inline(always)]
    pub fn txfifo_timeout(&mut self) -> TxfifoTimeoutW<'_, HungConfSpec> {
        TxfifoTimeoutW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Configures the maximum counter value."]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&mut self) -> TxfifoTimeoutShiftW<'_, HungConfSpec> {
        TxfifoTimeoutShiftW::new(self, 8)
    }
    #[doc = "Bit 11 - Set this bit to enable TX FIFO timeout when receiving."]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&mut self) -> TxfifoTimeoutEnaW<'_, HungConfSpec> {
        TxfifoTimeoutEnaW::new(self, 11)
    }
    #[doc = "Bits 12:19 - Stores the timeout value. DMA generates UHCI_TX_HUNG_INT for timeout when reading RAM data."]
    #[inline(always)]
    pub fn rxfifo_timeout(&mut self) -> RxfifoTimeoutW<'_, HungConfSpec> {
        RxfifoTimeoutW::new(self, 12)
    }
    #[doc = "Bits 20:22 - Configures the maximum counter value."]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&mut self) -> RxfifoTimeoutShiftW<'_, HungConfSpec> {
        RxfifoTimeoutShiftW::new(self, 20)
    }
    #[doc = "Bit 23 - Set this bit to enable TX FIFO timeout when DMA sending data."]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&mut self) -> RxfifoTimeoutEnaW<'_, HungConfSpec> {
        RxfifoTimeoutEnaW::new(self, 23)
    }
}
#[doc = "UHCI Hung Configuration Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`hung_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hung_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HungConfSpec;
impl crate::RegisterSpec for HungConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hung_conf::R`](R) reader structure"]
impl crate::Readable for HungConfSpec {}
#[doc = "`write(|w| ..)` method takes [`hung_conf::W`](W) writer structure"]
impl crate::Writable for HungConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUNG_CONF to value 0x0081_0810"]
impl crate::Resettable for HungConfSpec {
    const RESET_VALUE: u32 = 0x0081_0810;
}

#[doc = "Register `LC_HUNG_CONF` reader"]
pub type R = crate::R<LcHungConfSpec>;
#[doc = "Register `LC_HUNG_CONF` writer"]
pub type W = crate::W<LcHungConfSpec>;
#[doc = "Field `LC_FIFO_TIMEOUT` reader - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
pub type LcFifoTimeoutR = crate::FieldReader;
#[doc = "Field `LC_FIFO_TIMEOUT` writer - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
pub type LcFifoTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` reader - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
pub type LcFifoTimeoutShiftR = crate::FieldReader;
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` writer - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
pub type LcFifoTimeoutShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` reader - The enable bit for FIFO timeout"]
pub type LcFifoTimeoutEnaR = crate::BitReader;
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` writer - The enable bit for FIFO timeout"]
pub type LcFifoTimeoutEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
    #[inline(always)]
    pub fn lc_fifo_timeout(&self) -> LcFifoTimeoutR {
        LcFifoTimeoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
    #[inline(always)]
    pub fn lc_fifo_timeout_shift(&self) -> LcFifoTimeoutShiftR {
        LcFifoTimeoutShiftR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout"]
    #[inline(always)]
    pub fn lc_fifo_timeout_ena(&self) -> LcFifoTimeoutEnaR {
        LcFifoTimeoutEnaR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
    #[inline(always)]
    pub fn lc_fifo_timeout(&mut self) -> LcFifoTimeoutW<'_, LcHungConfSpec> {
        LcFifoTimeoutW::new(self, 0)
    }
    #[doc = "Bits 8:10 - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
    #[inline(always)]
    pub fn lc_fifo_timeout_shift(&mut self) -> LcFifoTimeoutShiftW<'_, LcHungConfSpec> {
        LcFifoTimeoutShiftW::new(self, 8)
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout"]
    #[inline(always)]
    pub fn lc_fifo_timeout_ena(&mut self) -> LcFifoTimeoutEnaW<'_, LcHungConfSpec> {
        LcFifoTimeoutEnaW::new(self, 11)
    }
}
#[doc = "I2S HUNG configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_hung_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc_hung_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcHungConfSpec;
impl crate::RegisterSpec for LcHungConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_hung_conf::R`](R) reader structure"]
impl crate::Readable for LcHungConfSpec {}
#[doc = "`write(|w| ..)` method takes [`lc_hung_conf::W`](W) writer structure"]
impl crate::Writable for LcHungConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LC_HUNG_CONF to value 0x0810"]
impl crate::Resettable for LcHungConfSpec {
    const RESET_VALUE: u32 = 0x0810;
}

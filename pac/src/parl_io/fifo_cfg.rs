#[doc = "Register `FIFO_CFG` reader"]
pub type R = crate::R<FifoCfgSpec>;
#[doc = "Register `FIFO_CFG` writer"]
pub type W = crate::W<FifoCfgSpec>;
#[doc = "Field `TX_FIFO_SRST` reader - Write 1 to reset async fifo in tx module."]
pub type TxFifoSrstR = crate::BitReader;
#[doc = "Field `TX_FIFO_SRST` writer - Write 1 to reset async fifo in tx module."]
pub type TxFifoSrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_SRST` reader - Write 1 to reset async fifo in rx module."]
pub type RxFifoSrstR = crate::BitReader;
#[doc = "Field `RX_FIFO_SRST` writer - Write 1 to reset async fifo in rx module."]
pub type RxFifoSrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Write 1 to reset async fifo in tx module."]
    #[inline(always)]
    pub fn tx_fifo_srst(&self) -> TxFifoSrstR {
        TxFifoSrstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1 to reset async fifo in rx module."]
    #[inline(always)]
    pub fn rx_fifo_srst(&self) -> RxFifoSrstR {
        RxFifoSrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Write 1 to reset async fifo in tx module."]
    #[inline(always)]
    pub fn tx_fifo_srst(&mut self) -> TxFifoSrstW<'_, FifoCfgSpec> {
        TxFifoSrstW::new(self, 30)
    }
    #[doc = "Bit 31 - Write 1 to reset async fifo in rx module."]
    #[inline(always)]
    pub fn rx_fifo_srst(&mut self) -> RxFifoSrstW<'_, FifoCfgSpec> {
        RxFifoSrstW::new(self, 31)
    }
}
#[doc = "Parallel IO FIFO configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoCfgSpec;
impl crate::RegisterSpec for FifoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_cfg::R`](R) reader structure"]
impl crate::Readable for FifoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_cfg::W`](W) writer structure"]
impl crate::Writable for FifoCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CFG to value 0"]
impl crate::Resettable for FifoCfgSpec {}

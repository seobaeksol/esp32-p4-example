#[doc = "Register `INTSET` reader"]
pub type R = crate::R<IntsetSpec>;
#[doc = "Register `INTSET` writer"]
pub type W = crate::W<IntsetSpec>;
#[doc = "Field `STOP_ENA` reader - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
pub type StopEnaR = crate::BitReader;
#[doc = "Field `STOP_ENA` writer - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
pub type StopEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND_ENA` reader - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
pub type RxpendEnaR = crate::BitReader;
#[doc = "Field `RXPEND_ENA` writer - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
pub type RxpendEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSEND_ENA` reader - NA"]
pub type TxsendEnaR = crate::BitReader;
#[doc = "Field `TXSEND_ENA` writer - NA"]
pub type TxsendEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
    #[inline(always)]
    pub fn stop_ena(&self) -> StopEnaR {
        StopEnaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
    #[inline(always)]
    pub fn rxpend_ena(&self) -> RxpendEnaR {
        RxpendEnaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn txsend_ena(&self) -> TxsendEnaR {
        TxsendEnaR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
    #[inline(always)]
    pub fn stop_ena(&mut self) -> StopEnaW<'_, IntsetSpec> {
        StopEnaW::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
    #[inline(always)]
    pub fn rxpend_ena(&mut self) -> RxpendEnaW<'_, IntsetSpec> {
        RxpendEnaW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn txsend_ena(&mut self) -> TxsendEnaW<'_, IntsetSpec> {
        TxsendEnaW::new(self, 12)
    }
}
#[doc = "INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)\n\nYou can [`read`](crate::Reg::read) this register and get [`intset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsetSpec;
impl crate::RegisterSpec for IntsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intset::R`](R) reader structure"]
impl crate::Readable for IntsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intset::W`](W) writer structure"]
impl crate::Writable for IntsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for IntsetSpec {}

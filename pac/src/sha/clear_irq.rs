#[doc = "Register `CLEAR_IRQ` writer"]
pub type W = crate::W<ClearIrqSpec>;
#[doc = "Field `CLEAR_INTERRUPT` writer - Write 1 to clear DMA-SHA interrupt."]
pub type ClearInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear DMA-SHA interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(&mut self) -> ClearInterruptW<'_, ClearIrqSpec> {
        ClearInterruptW::new(self, 0)
    }
}
#[doc = "DMA-SHA interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_irq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearIrqSpec;
impl crate::RegisterSpec for ClearIrqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear_irq::W`](W) writer structure"]
impl crate::Writable for ClearIrqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEAR_IRQ to value 0"]
impl crate::Resettable for ClearIrqSpec {}

#[doc = "Register `INTMASKED` reader"]
pub type R = crate::R<IntmaskedSpec>;
#[doc = "Field `STOP_MASK` reader - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
pub type StopMaskR = crate::BitReader;
#[doc = "Field `RXPEND_MASK` reader - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
pub type RxpendMaskR = crate::BitReader;
#[doc = "Field `TXSEND_MASK` reader - NA"]
pub type TxsendMaskR = crate::BitReader;
impl R {
    #[doc = "Bit 10 - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
    #[inline(always)]
    pub fn stop_mask(&self) -> StopMaskR {
        StopMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
    #[inline(always)]
    pub fn rxpend_mask(&self) -> RxpendMaskR {
        RxpendMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn txsend_mask(&self) -> TxsendMaskR {
        TxsendMaskR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intmasked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmaskedSpec;
impl crate::RegisterSpec for IntmaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmasked::R`](R) reader structure"]
impl crate::Readable for IntmaskedSpec {}
#[doc = "`reset()` method sets INTMASKED to value 0"]
impl crate::Resettable for IntmaskedSpec {}

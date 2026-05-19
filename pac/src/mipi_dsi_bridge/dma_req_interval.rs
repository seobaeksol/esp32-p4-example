#[doc = "Register `DMA_REQ_INTERVAL` reader"]
pub type R = crate::R<DmaReqIntervalSpec>;
#[doc = "Register `DMA_REQ_INTERVAL` writer"]
pub type W = crate::W<DmaReqIntervalSpec>;
#[doc = "Field `DMA_REQ_INTERVAL` reader - this field configures the interval between dma req events"]
pub type DmaReqIntervalR = crate::FieldReader<u16>;
#[doc = "Field `DMA_REQ_INTERVAL` writer - this field configures the interval between dma req events"]
pub type DmaReqIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - this field configures the interval between dma req events"]
    #[inline(always)]
    pub fn dma_req_interval(&self) -> DmaReqIntervalR {
        DmaReqIntervalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - this field configures the interval between dma req events"]
    #[inline(always)]
    pub fn dma_req_interval(&mut self) -> DmaReqIntervalW<'_, DmaReqIntervalSpec> {
        DmaReqIntervalW::new(self, 0)
    }
}
#[doc = "dsi bridge dma req interval control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_req_interval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_req_interval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaReqIntervalSpec;
impl crate::RegisterSpec for DmaReqIntervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_interval::R`](R) reader structure"]
impl crate::Readable for DmaReqIntervalSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_req_interval::W`](W) writer structure"]
impl crate::Writable for DmaReqIntervalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_REQ_INTERVAL to value 0x01"]
impl crate::Resettable for DmaReqIntervalSpec {
    const RESET_VALUE: u32 = 0x01;
}

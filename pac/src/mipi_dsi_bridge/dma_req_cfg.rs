#[doc = "Register `DMA_REQ_CFG` reader"]
pub type R = crate::R<DmaReqCfgSpec>;
#[doc = "Register `DMA_REQ_CFG` writer"]
pub type W = crate::W<DmaReqCfgSpec>;
#[doc = "Field `DMA_BURST_LEN` reader - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
pub type DmaBurstLenR = crate::FieldReader<u16>;
#[doc = "Field `DMA_BURST_LEN` writer - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
pub type DmaBurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn dma_burst_len(&self) -> DmaBurstLenR {
        DmaBurstLenR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn dma_burst_len(&mut self) -> DmaBurstLenW<'_, DmaReqCfgSpec> {
        DmaBurstLenW::new(self, 0)
    }
}
#[doc = "dsi bridge dma burst len register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_req_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_req_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaReqCfgSpec;
impl crate::RegisterSpec for DmaReqCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_cfg::R`](R) reader structure"]
impl crate::Readable for DmaReqCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_req_cfg::W`](W) writer structure"]
impl crate::Writable for DmaReqCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_REQ_CFG to value 0x80"]
impl crate::Resettable for DmaReqCfgSpec {
    const RESET_VALUE: u32 = 0x80;
}

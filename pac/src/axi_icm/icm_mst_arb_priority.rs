#[doc = "Register `ICM_MST_ARB_PRIORITY` reader"]
pub type R = crate::R<IcmMstArbPrioritySpec>;
#[doc = "Register `ICM_MST_ARB_PRIORITY` writer"]
pub type W = crate::W<IcmMstArbPrioritySpec>;
#[doc = "Field `CPU_PRIORITY` reader - "]
pub type CpuPriorityR = crate::FieldReader;
#[doc = "Field `CPU_PRIORITY` writer - "]
pub type CpuPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CACHE_PRIORITY` reader - "]
pub type CachePriorityR = crate::FieldReader;
#[doc = "Field `CACHE_PRIORITY` writer - "]
pub type CachePriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMA2D_PRIORITY` reader - "]
pub type Dma2dPriorityR = crate::FieldReader;
#[doc = "Field `DMA2D_PRIORITY` writer - "]
pub type Dma2dPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST1_PRIORITY` reader - "]
pub type GdmaMst1PriorityR = crate::FieldReader;
#[doc = "Field `GDMA_MST1_PRIORITY` writer - "]
pub type GdmaMst1PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST2_PRIORITY` reader - "]
pub type GdmaMst2PriorityR = crate::FieldReader;
#[doc = "Field `GDMA_MST2_PRIORITY` writer - "]
pub type GdmaMst2PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_M1_PRIORITY` reader - "]
pub type H264M1PriorityR = crate::FieldReader;
#[doc = "Field `H264_M1_PRIORITY` writer - "]
pub type H264M1PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_M2_PRIORITY` reader - "]
pub type H264M2PriorityR = crate::FieldReader;
#[doc = "Field `H264_M2_PRIORITY` writer - "]
pub type H264M2PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AXI_PDMA_PRIORITY` reader - "]
pub type AxiPdmaPriorityR = crate::FieldReader;
#[doc = "Field `AXI_PDMA_PRIORITY` writer - "]
pub type AxiPdmaPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_priority(&self) -> CpuPriorityR {
        CpuPriorityR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_priority(&self) -> CachePriorityR {
        CachePriorityR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_priority(&self) -> Dma2dPriorityR {
        Dma2dPriorityR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_priority(&self) -> GdmaMst1PriorityR {
        GdmaMst1PriorityR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_priority(&self) -> GdmaMst2PriorityR {
        GdmaMst2PriorityR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_m1_priority(&self) -> H264M1PriorityR {
        H264M1PriorityR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_m2_priority(&self) -> H264M2PriorityR {
        H264M2PriorityR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn axi_pdma_priority(&self) -> AxiPdmaPriorityR {
        AxiPdmaPriorityR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_priority(&mut self) -> CpuPriorityW<'_, IcmMstArbPrioritySpec> {
        CpuPriorityW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_priority(&mut self) -> CachePriorityW<'_, IcmMstArbPrioritySpec> {
        CachePriorityW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_priority(&mut self) -> Dma2dPriorityW<'_, IcmMstArbPrioritySpec> {
        Dma2dPriorityW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_priority(&mut self) -> GdmaMst1PriorityW<'_, IcmMstArbPrioritySpec> {
        GdmaMst1PriorityW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_priority(&mut self) -> GdmaMst2PriorityW<'_, IcmMstArbPrioritySpec> {
        GdmaMst2PriorityW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_m1_priority(&mut self) -> H264M1PriorityW<'_, IcmMstArbPrioritySpec> {
        H264M1PriorityW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_m2_priority(&mut self) -> H264M2PriorityW<'_, IcmMstArbPrioritySpec> {
        H264M2PriorityW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn axi_pdma_priority(&mut self) -> AxiPdmaPriorityW<'_, IcmMstArbPrioritySpec> {
        AxiPdmaPriorityW::new(self, 28)
    }
}
#[doc = "Master arbitration priority\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_mst_arb_priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_mst_arb_priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmMstArbPrioritySpec;
impl crate::RegisterSpec for IcmMstArbPrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_mst_arb_priority::R`](R) reader structure"]
impl crate::Readable for IcmMstArbPrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`icm_mst_arb_priority::W`](W) writer structure"]
impl crate::Writable for IcmMstArbPrioritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_MST_ARB_PRIORITY to value 0"]
impl crate::Resettable for IcmMstArbPrioritySpec {}

#[doc = "Register `ICM_MST_AWQOS` reader"]
pub type R = crate::R<IcmMstAwqosSpec>;
#[doc = "Register `ICM_MST_AWQOS` writer"]
pub type W = crate::W<IcmMstAwqosSpec>;
#[doc = "Field `CPU_AWQOS` reader - "]
pub type CpuAwqosR = crate::FieldReader;
#[doc = "Field `CPU_AWQOS` writer - "]
pub type CpuAwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CACHE_AWQOS` reader - "]
pub type CacheAwqosR = crate::FieldReader;
#[doc = "Field `CACHE_AWQOS` writer - "]
pub type CacheAwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMA2D_AWQOS` reader - "]
pub type Dma2dAwqosR = crate::FieldReader;
#[doc = "Field `DMA2D_AWQOS` writer - "]
pub type Dma2dAwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST1_AWQOS` reader - "]
pub type GdmaMst1AwqosR = crate::FieldReader;
#[doc = "Field `GDMA_MST1_AWQOS` writer - "]
pub type GdmaMst1AwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST2_AWQOS` reader - "]
pub type GdmaMst2AwqosR = crate::FieldReader;
#[doc = "Field `GDMA_MST2_AWQOS` writer - "]
pub type GdmaMst2AwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_DMA2D_M1_AWQOS` reader - "]
pub type H264Dma2dM1AwqosR = crate::FieldReader;
#[doc = "Field `H264_DMA2D_M1_AWQOS` writer - "]
pub type H264Dma2dM1AwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_DMA2D_M2_AWQOS` reader - "]
pub type H264Dma2dM2AwqosR = crate::FieldReader;
#[doc = "Field `H264_DMA2D_M2_AWQOS` writer - "]
pub type H264Dma2dM2AwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDMA_INT_AWQOS` reader - "]
pub type PdmaIntAwqosR = crate::FieldReader;
#[doc = "Field `PDMA_INT_AWQOS` writer - "]
pub type PdmaIntAwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_awqos(&self) -> CpuAwqosR {
        CpuAwqosR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_awqos(&self) -> CacheAwqosR {
        CacheAwqosR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_awqos(&self) -> Dma2dAwqosR {
        Dma2dAwqosR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_awqos(&self) -> GdmaMst1AwqosR {
        GdmaMst1AwqosR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_awqos(&self) -> GdmaMst2AwqosR {
        GdmaMst2AwqosR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_dma2d_m1_awqos(&self) -> H264Dma2dM1AwqosR {
        H264Dma2dM1AwqosR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_dma2d_m2_awqos(&self) -> H264Dma2dM2AwqosR {
        H264Dma2dM2AwqosR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pdma_int_awqos(&self) -> PdmaIntAwqosR {
        PdmaIntAwqosR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_awqos(&mut self) -> CpuAwqosW<'_, IcmMstAwqosSpec> {
        CpuAwqosW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_awqos(&mut self) -> CacheAwqosW<'_, IcmMstAwqosSpec> {
        CacheAwqosW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_awqos(&mut self) -> Dma2dAwqosW<'_, IcmMstAwqosSpec> {
        Dma2dAwqosW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_awqos(&mut self) -> GdmaMst1AwqosW<'_, IcmMstAwqosSpec> {
        GdmaMst1AwqosW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_awqos(&mut self) -> GdmaMst2AwqosW<'_, IcmMstAwqosSpec> {
        GdmaMst2AwqosW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_dma2d_m1_awqos(&mut self) -> H264Dma2dM1AwqosW<'_, IcmMstAwqosSpec> {
        H264Dma2dM1AwqosW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_dma2d_m2_awqos(&mut self) -> H264Dma2dM2AwqosW<'_, IcmMstAwqosSpec> {
        H264Dma2dM2AwqosW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pdma_int_awqos(&mut self) -> PdmaIntAwqosW<'_, IcmMstAwqosSpec> {
        PdmaIntAwqosW::new(self, 28)
    }
}
#[doc = "Master write QoS\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_mst_awqos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_mst_awqos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmMstAwqosSpec;
impl crate::RegisterSpec for IcmMstAwqosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_mst_awqos::R`](R) reader structure"]
impl crate::Readable for IcmMstAwqosSpec {}
#[doc = "`write(|w| ..)` method takes [`icm_mst_awqos::W`](W) writer structure"]
impl crate::Writable for IcmMstAwqosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_MST_AWQOS to value 0"]
impl crate::Resettable for IcmMstAwqosSpec {}

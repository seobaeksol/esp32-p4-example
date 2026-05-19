#[doc = "Register `VPU_CTRL` reader"]
pub type R = crate::R<VpuCtrlSpec>;
#[doc = "Register `VPU_CTRL` writer"]
pub type W = crate::W<VpuCtrlSpec>;
#[doc = "Field `PPA_LSLP_MEM_PD` reader - N/A"]
pub type PpaLslpMemPdR = crate::BitReader;
#[doc = "Field `PPA_LSLP_MEM_PD` writer - N/A"]
pub type PpaLslpMemPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG_SDSLP_MEM_PD` reader - N/A"]
pub type JpegSdslpMemPdR = crate::BitReader;
#[doc = "Field `JPEG_SDSLP_MEM_PD` writer - N/A"]
pub type JpegSdslpMemPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG_LSLP_MEM_PD` reader - N/A"]
pub type JpegLslpMemPdR = crate::BitReader;
#[doc = "Field `JPEG_LSLP_MEM_PD` writer - N/A"]
pub type JpegLslpMemPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG_DSLP_MEM_PD` reader - N/A"]
pub type JpegDslpMemPdR = crate::BitReader;
#[doc = "Field `JPEG_DSLP_MEM_PD` writer - N/A"]
pub type JpegDslpMemPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_LSLP_MEM_PD` reader - N/A"]
pub type Dma2dLslpMemPdR = crate::BitReader;
#[doc = "Field `DMA2D_LSLP_MEM_PD` writer - N/A"]
pub type Dma2dLslpMemPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn ppa_lslp_mem_pd(&self) -> PpaLslpMemPdR {
        PpaLslpMemPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn jpeg_sdslp_mem_pd(&self) -> JpegSdslpMemPdR {
        JpegSdslpMemPdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn jpeg_lslp_mem_pd(&self) -> JpegLslpMemPdR {
        JpegLslpMemPdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn jpeg_dslp_mem_pd(&self) -> JpegDslpMemPdR {
        JpegDslpMemPdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dma2d_lslp_mem_pd(&self) -> Dma2dLslpMemPdR {
        Dma2dLslpMemPdR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn ppa_lslp_mem_pd(&mut self) -> PpaLslpMemPdW<'_, VpuCtrlSpec> {
        PpaLslpMemPdW::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn jpeg_sdslp_mem_pd(&mut self) -> JpegSdslpMemPdW<'_, VpuCtrlSpec> {
        JpegSdslpMemPdW::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn jpeg_lslp_mem_pd(&mut self) -> JpegLslpMemPdW<'_, VpuCtrlSpec> {
        JpegLslpMemPdW::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn jpeg_dslp_mem_pd(&mut self) -> JpegDslpMemPdW<'_, VpuCtrlSpec> {
        JpegDslpMemPdW::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dma2d_lslp_mem_pd(&mut self) -> Dma2dLslpMemPdW<'_, VpuCtrlSpec> {
        Dma2dLslpMemPdW::new(self, 4)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`vpu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vpu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VpuCtrlSpec;
impl crate::RegisterSpec for VpuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vpu_ctrl::R`](R) reader structure"]
impl crate::Readable for VpuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vpu_ctrl::W`](W) writer structure"]
impl crate::Writable for VpuCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VPU_CTRL to value 0"]
impl crate::Resettable for VpuCtrlSpec {}

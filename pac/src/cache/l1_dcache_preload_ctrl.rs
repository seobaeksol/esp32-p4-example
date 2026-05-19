#[doc = "Register `L1_DCACHE_PRELOAD_CTRL` reader"]
pub type R = crate::R<L1DcachePreloadCtrlSpec>;
#[doc = "Register `L1_DCACHE_PRELOAD_CTRL` writer"]
pub type W = crate::W<L1DcachePreloadCtrlSpec>;
#[doc = "Field `L1_DCACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation on L1-DCache. It will be cleared by hardware automatically after preload operation is done."]
pub type L1DcachePreloadEnaR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation on L1-DCache. It will be cleared by hardware automatically after preload operation is done."]
pub type L1DcachePreloadEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_PRELOAD_DONE` reader - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
pub type L1DcachePreloadDoneR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
pub type L1DcachePreloadOrderR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOAD_ORDER` writer - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
pub type L1DcachePreloadOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_PRELOAD_RGID` reader - The bit is used to set the gid of l1 dcache preload."]
pub type L1DcachePreloadRgidR = crate::FieldReader;
#[doc = "Field `L1_DCACHE_PRELOAD_RGID` writer - The bit is used to set the gid of l1 dcache preload."]
pub type L1DcachePreloadRgidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L1_DCACHE_PRELOAD_MODE` reader - The bit is used to configure the mode of l1 dcache preload, 0: load data from next level memory, 1: not load data from next level memory."]
pub type L1DcachePreloadModeR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOAD_MODE` writer - The bit is used to configure the mode of l1 dcache preload, 0: load data from next level memory, 1: not load data from next level memory."]
pub type L1DcachePreloadModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L1-DCache. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    pub fn l1_dcache_preload_ena(&self) -> L1DcachePreloadEnaR {
        L1DcachePreloadEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_dcache_preload_done(&self) -> L1DcachePreloadDoneR {
        L1DcachePreloadDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
    #[inline(always)]
    pub fn l1_dcache_preload_order(&self) -> L1DcachePreloadOrderR {
        L1DcachePreloadOrderR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of l1 dcache preload."]
    #[inline(always)]
    pub fn l1_dcache_preload_rgid(&self) -> L1DcachePreloadRgidR {
        L1DcachePreloadRgidR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - The bit is used to configure the mode of l1 dcache preload, 0: load data from next level memory, 1: not load data from next level memory."]
    #[inline(always)]
    pub fn l1_dcache_preload_mode(&self) -> L1DcachePreloadModeR {
        L1DcachePreloadModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L1-DCache. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    pub fn l1_dcache_preload_ena(&mut self) -> L1DcachePreloadEnaW<'_, L1DcachePreloadCtrlSpec> {
        L1DcachePreloadEnaW::new(self, 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
    #[inline(always)]
    pub fn l1_dcache_preload_order(
        &mut self,
    ) -> L1DcachePreloadOrderW<'_, L1DcachePreloadCtrlSpec> {
        L1DcachePreloadOrderW::new(self, 2)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of l1 dcache preload."]
    #[inline(always)]
    pub fn l1_dcache_preload_rgid(&mut self) -> L1DcachePreloadRgidW<'_, L1DcachePreloadCtrlSpec> {
        L1DcachePreloadRgidW::new(self, 3)
    }
    #[doc = "Bit 7 - The bit is used to configure the mode of l1 dcache preload, 0: load data from next level memory, 1: not load data from next level memory."]
    #[inline(always)]
    pub fn l1_dcache_preload_mode(&mut self) -> L1DcachePreloadModeW<'_, L1DcachePreloadCtrlSpec> {
        L1DcachePreloadModeW::new(self, 7)
    }
}
#[doc = "L1 data Cache preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_preload_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_preload_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcachePreloadCtrlSpec;
impl crate::RegisterSpec for L1DcachePreloadCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_preload_ctrl::R`](R) reader structure"]
impl crate::Readable for L1DcachePreloadCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_preload_ctrl::W`](W) writer structure"]
impl crate::Writable for L1DcachePreloadCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_PRELOAD_CTRL to value 0x02"]
impl crate::Resettable for L1DcachePreloadCtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}

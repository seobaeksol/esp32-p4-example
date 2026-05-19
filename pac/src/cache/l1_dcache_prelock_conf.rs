#[doc = "Register `L1_DCACHE_PRELOCK_CONF` reader"]
pub type R = crate::R<L1DcachePrelockConfSpec>;
#[doc = "Register `L1_DCACHE_PRELOCK_CONF` writer"]
pub type W = crate::W<L1DcachePrelockConfSpec>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L1-DCache."]
pub type L1DcachePrelockSct0EnR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function on L1-DCache."]
pub type L1DcachePrelockSct0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L1-DCache."]
pub type L1DcachePrelockSct1EnR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function on L1-DCache."]
pub type L1DcachePrelockSct1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_PRELOCK_RGID` reader - The bit is used to set the gid of l1 dcache prelock."]
pub type L1DcachePrelockRgidR = crate::FieldReader;
#[doc = "Field `L1_DCACHE_PRELOCK_RGID` writer - The bit is used to set the gid of l1 dcache prelock."]
pub type L1DcachePrelockRgidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct0_en(&self) -> L1DcachePrelockSct0EnR {
        L1DcachePrelockSct0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct1_en(&self) -> L1DcachePrelockSct1EnR {
        L1DcachePrelockSct1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 dcache prelock."]
    #[inline(always)]
    pub fn l1_dcache_prelock_rgid(&self) -> L1DcachePrelockRgidR {
        L1DcachePrelockRgidR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct0_en(
        &mut self,
    ) -> L1DcachePrelockSct0EnW<'_, L1DcachePrelockConfSpec> {
        L1DcachePrelockSct0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct1_en(
        &mut self,
    ) -> L1DcachePrelockSct1EnW<'_, L1DcachePrelockConfSpec> {
        L1DcachePrelockSct1EnW::new(self, 1)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 dcache prelock."]
    #[inline(always)]
    pub fn l1_dcache_prelock_rgid(&mut self) -> L1DcachePrelockRgidW<'_, L1DcachePrelockConfSpec> {
        L1DcachePrelockRgidW::new(self, 2)
    }
}
#[doc = "L1 data Cache prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_prelock_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_prelock_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcachePrelockConfSpec;
impl crate::RegisterSpec for L1DcachePrelockConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_prelock_conf::R`](R) reader structure"]
impl crate::Readable for L1DcachePrelockConfSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_prelock_conf::W`](W) writer structure"]
impl crate::Writable for L1DcachePrelockConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_PRELOCK_CONF to value 0"]
impl crate::Resettable for L1DcachePrelockConfSpec {}

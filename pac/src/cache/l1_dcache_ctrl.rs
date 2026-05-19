#[doc = "Register `L1_DCACHE_CTRL` reader"]
pub type R = crate::R<L1DcacheCtrlSpec>;
#[doc = "Register `L1_DCACHE_CTRL` writer"]
pub type W = crate::W<L1DcacheCtrlSpec>;
#[doc = "Field `L1_DCACHE_SHUT_DBUS0` reader - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1DcacheShutDbus0R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DBUS0` writer - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1DcacheShutDbus0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_SHUT_DBUS1` reader - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1DcacheShutDbus1R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DBUS1` writer - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1DcacheShutDbus1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_SHUT_DBUS2` reader - Reserved"]
pub type L1DcacheShutDbus2R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DBUS3` reader - Reserved"]
pub type L1DcacheShutDbus3R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DMA` reader - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
pub type L1DcacheShutDmaR = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DMA` writer - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
pub type L1DcacheShutDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_UNDEF_OP` reader - Reserved"]
pub type L1DcacheUndefOpR = crate::FieldReader;
#[doc = "Field `L1_DCACHE_UNDEF_OP` writer - Reserved"]
pub type L1DcacheUndefOpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus0(&self) -> L1DcacheShutDbus0R {
        L1DcacheShutDbus0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus1(&self) -> L1DcacheShutDbus1R {
        L1DcacheShutDbus1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus2(&self) -> L1DcacheShutDbus2R {
        L1DcacheShutDbus2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus3(&self) -> L1DcacheShutDbus3R {
        L1DcacheShutDbus3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dma(&self) -> L1DcacheShutDmaR {
        L1DcacheShutDmaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_undef_op(&self) -> L1DcacheUndefOpR {
        L1DcacheUndefOpR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus0(&mut self) -> L1DcacheShutDbus0W<'_, L1DcacheCtrlSpec> {
        L1DcacheShutDbus0W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus1(&mut self) -> L1DcacheShutDbus1W<'_, L1DcacheCtrlSpec> {
        L1DcacheShutDbus1W::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dma(&mut self) -> L1DcacheShutDmaW<'_, L1DcacheCtrlSpec> {
        L1DcacheShutDmaW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_undef_op(&mut self) -> L1DcacheUndefOpW<'_, L1DcacheCtrlSpec> {
        L1DcacheUndefOpW::new(self, 8)
    }
}
#[doc = "L1 data Cache(L1-DCache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcacheCtrlSpec;
impl crate::RegisterSpec for L1DcacheCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_ctrl::R`](R) reader structure"]
impl crate::Readable for L1DcacheCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_ctrl::W`](W) writer structure"]
impl crate::Writable for L1DcacheCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_CTRL to value 0"]
impl crate::Resettable for L1DcacheCtrlSpec {}

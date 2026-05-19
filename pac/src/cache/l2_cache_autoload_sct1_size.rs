#[doc = "Register `L2_CACHE_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<L2CacheAutoloadSct1SizeSpec>;
#[doc = "Register `L2_CACHE_AUTOLOAD_SCT1_SIZE` writer"]
pub type W = crate::W<L2CacheAutoloadSct1SizeSpec>;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT1_ADDR and L2_CACHE_AUTOLOAD_SCT1_ENA."]
pub type L2CacheAutoloadSct1SizeR = crate::FieldReader<u32>;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT1_SIZE` writer - Those bits are used to configure the size of the second section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT1_ADDR and L2_CACHE_AUTOLOAD_SCT1_ENA."]
pub type L2CacheAutoloadSct1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT1_ADDR and L2_CACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct1_size(&self) -> L2CacheAutoloadSct1SizeR {
        L2CacheAutoloadSct1SizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT1_ADDR and L2_CACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct1_size(
        &mut self,
    ) -> L2CacheAutoloadSct1SizeW<'_, L2CacheAutoloadSct1SizeSpec> {
        L2CacheAutoloadSct1SizeW::new(self, 0)
    }
}
#[doc = "L2 Cache autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_autoload_sct1_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_autoload_sct1_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheAutoloadSct1SizeSpec;
impl crate::RegisterSpec for L2CacheAutoloadSct1SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for L2CacheAutoloadSct1SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_autoload_sct1_size::W`](W) writer structure"]
impl crate::Writable for L2CacheAutoloadSct1SizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for L2CacheAutoloadSct1SizeSpec {}

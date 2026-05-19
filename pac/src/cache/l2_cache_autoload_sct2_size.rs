#[doc = "Register `L2_CACHE_AUTOLOAD_SCT2_SIZE` reader"]
pub type R = crate::R<L2CacheAutoloadSct2SizeSpec>;
#[doc = "Register `L2_CACHE_AUTOLOAD_SCT2_SIZE` writer"]
pub type W = crate::W<L2CacheAutoloadSct2SizeSpec>;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT2_SIZE` reader - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA."]
pub type L2CacheAutoloadSct2SizeR = crate::FieldReader<u32>;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT2_SIZE` writer - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA."]
pub type L2CacheAutoloadSct2SizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct2_size(&self) -> L2CacheAutoloadSct2SizeR {
        L2CacheAutoloadSct2SizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct2_size(
        &mut self,
    ) -> L2CacheAutoloadSct2SizeW<'_, L2CacheAutoloadSct2SizeSpec> {
        L2CacheAutoloadSct2SizeW::new(self, 0)
    }
}
#[doc = "L2 Cache autoload section 2 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_autoload_sct2_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_autoload_sct2_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheAutoloadSct2SizeSpec;
impl crate::RegisterSpec for L2CacheAutoloadSct2SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_autoload_sct2_size::R`](R) reader structure"]
impl crate::Readable for L2CacheAutoloadSct2SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_autoload_sct2_size::W`](W) writer structure"]
impl crate::Writable for L2CacheAutoloadSct2SizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_SCT2_SIZE to value 0"]
impl crate::Resettable for L2CacheAutoloadSct2SizeSpec {}

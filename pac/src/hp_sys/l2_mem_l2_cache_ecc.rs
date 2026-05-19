#[doc = "Register `L2_MEM_L2_CACHE_ECC` reader"]
pub type R = crate::R<L2MemL2CacheEccSpec>;
#[doc = "Register `L2_MEM_L2_CACHE_ECC` writer"]
pub type W = crate::W<L2MemL2CacheEccSpec>;
#[doc = "Field `REG_L2_CACHE_ECC_EN` reader - NA"]
pub type RegL2CacheEccEnR = crate::BitReader;
#[doc = "Field `REG_L2_CACHE_ECC_EN` writer - NA"]
pub type RegL2CacheEccEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_cache_ecc_en(&self) -> RegL2CacheEccEnR {
        RegL2CacheEccEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_cache_ecc_en(&mut self) -> RegL2CacheEccEnW<'_, L2MemL2CacheEccSpec> {
        RegL2CacheEccEnW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_l2_cache_ecc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_l2_cache_ecc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2MemL2CacheEccSpec;
impl crate::RegisterSpec for L2MemL2CacheEccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_l2_cache_ecc::R`](R) reader structure"]
impl crate::Readable for L2MemL2CacheEccSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_l2_cache_ecc::W`](W) writer structure"]
impl crate::Writable for L2MemL2CacheEccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_L2_CACHE_ECC to value 0"]
impl crate::Resettable for L2MemL2CacheEccSpec {}

#[doc = "Register `CACHE_RESET_CONFIG` reader"]
pub type R = crate::R<CacheResetConfigSpec>;
#[doc = "Register `CACHE_RESET_CONFIG` writer"]
pub type W = crate::W<CacheResetConfigSpec>;
#[doc = "Field `REG_L1_D_CACHE_RESET` reader - set 1 to reset l1 dcahce"]
pub type RegL1DCacheResetR = crate::BitReader;
#[doc = "Field `REG_L1_D_CACHE_RESET` writer - set 1 to reset l1 dcahce"]
pub type RegL1DCacheResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L1_I1_CACHE_RESET` reader - set 1 to reset l1 icahce1"]
pub type RegL1I1CacheResetR = crate::BitReader;
#[doc = "Field `REG_L1_I1_CACHE_RESET` writer - set 1 to reset l1 icahce1"]
pub type RegL1I1CacheResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L1_I0_CACHE_RESET` reader - set 1 to reset l1 icahce0"]
pub type RegL1I0CacheResetR = crate::BitReader;
#[doc = "Field `REG_L1_I0_CACHE_RESET` writer - set 1 to reset l1 icahce0"]
pub type RegL1I0CacheResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - set 1 to reset l1 dcahce"]
    #[inline(always)]
    pub fn reg_l1_d_cache_reset(&self) -> RegL1DCacheResetR {
        RegL1DCacheResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - set 1 to reset l1 icahce1"]
    #[inline(always)]
    pub fn reg_l1_i1_cache_reset(&self) -> RegL1I1CacheResetR {
        RegL1I1CacheResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - set 1 to reset l1 icahce0"]
    #[inline(always)]
    pub fn reg_l1_i0_cache_reset(&self) -> RegL1I0CacheResetR {
        RegL1I0CacheResetR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - set 1 to reset l1 dcahce"]
    #[inline(always)]
    pub fn reg_l1_d_cache_reset(&mut self) -> RegL1DCacheResetW<'_, CacheResetConfigSpec> {
        RegL1DCacheResetW::new(self, 1)
    }
    #[doc = "Bit 4 - set 1 to reset l1 icahce1"]
    #[inline(always)]
    pub fn reg_l1_i1_cache_reset(&mut self) -> RegL1I1CacheResetW<'_, CacheResetConfigSpec> {
        RegL1I1CacheResetW::new(self, 4)
    }
    #[doc = "Bit 5 - set 1 to reset l1 icahce0"]
    #[inline(always)]
    pub fn reg_l1_i0_cache_reset(&mut self) -> RegL1I0CacheResetW<'_, CacheResetConfigSpec> {
        RegL1I0CacheResetW::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_reset_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_reset_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacheResetConfigSpec;
impl crate::RegisterSpec for CacheResetConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_reset_config::R`](R) reader structure"]
impl crate::Readable for CacheResetConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cache_reset_config::W`](W) writer structure"]
impl crate::Writable for CacheResetConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_RESET_CONFIG to value 0"]
impl crate::Resettable for CacheResetConfigSpec {}

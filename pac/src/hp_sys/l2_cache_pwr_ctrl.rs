#[doc = "Register `L2_CACHE_PWR_CTRL` reader"]
pub type R = crate::R<L2CachePwrCtrlSpec>;
#[doc = "Register `L2_CACHE_PWR_CTRL` writer"]
pub type W = crate::W<L2CachePwrCtrlSpec>;
#[doc = "Field `REG_L2_CACHE_MEM_FO` reader - need_des"]
pub type RegL2CacheMemFoR = crate::FieldReader;
#[doc = "Field `REG_L2_CACHE_MEM_FO` writer - need_des"]
pub type RegL2CacheMemFoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn reg_l2_cache_mem_fo(&self) -> RegL2CacheMemFoR {
        RegL2CacheMemFoR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn reg_l2_cache_mem_fo(&mut self) -> RegL2CacheMemFoW<'_, L2CachePwrCtrlSpec> {
        RegL2CacheMemFoW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_pwr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_pwr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CachePwrCtrlSpec;
impl crate::RegisterSpec for L2CachePwrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for L2CachePwrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for L2CachePwrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_PWR_CTRL to value 0"]
impl crate::Resettable for L2CachePwrCtrlSpec {}

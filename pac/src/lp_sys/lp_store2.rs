#[doc = "Register `LP_STORE2` reader"]
pub type R = crate::R<LpStore2Spec>;
#[doc = "Register `LP_STORE2` writer"]
pub type W = crate::W<LpStore2Spec>;
#[doc = "Field `LP_SCRATCH2` reader - need_des"]
pub type LpScratch2R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH2` writer - need_des"]
pub type LpScratch2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch2(&self) -> LpScratch2R {
        LpScratch2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch2(&mut self) -> LpScratch2W<'_, LpStore2Spec> {
        LpScratch2W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpStore2Spec;
impl crate::RegisterSpec for LpStore2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store2::R`](R) reader structure"]
impl crate::Readable for LpStore2Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_store2::W`](W) writer structure"]
impl crate::Writable for LpStore2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_STORE2 to value 0"]
impl crate::Resettable for LpStore2Spec {}

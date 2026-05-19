#[doc = "Register `LP_STORE1` reader"]
pub type R = crate::R<LpStore1Spec>;
#[doc = "Register `LP_STORE1` writer"]
pub type W = crate::W<LpStore1Spec>;
#[doc = "Field `LP_SCRATCH1` reader - need_des"]
pub type LpScratch1R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH1` writer - need_des"]
pub type LpScratch1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch1(&self) -> LpScratch1R {
        LpScratch1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch1(&mut self) -> LpScratch1W<'_, LpStore1Spec> {
        LpScratch1W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpStore1Spec;
impl crate::RegisterSpec for LpStore1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store1::R`](R) reader structure"]
impl crate::Readable for LpStore1Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_store1::W`](W) writer structure"]
impl crate::Writable for LpStore1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_STORE1 to value 0"]
impl crate::Resettable for LpStore1Spec {}

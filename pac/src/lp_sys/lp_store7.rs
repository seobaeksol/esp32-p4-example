#[doc = "Register `LP_STORE7` reader"]
pub type R = crate::R<LpStore7Spec>;
#[doc = "Register `LP_STORE7` writer"]
pub type W = crate::W<LpStore7Spec>;
#[doc = "Field `LP_SCRATCH7` reader - need_des"]
pub type LpScratch7R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH7` writer - need_des"]
pub type LpScratch7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch7(&self) -> LpScratch7R {
        LpScratch7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch7(&mut self) -> LpScratch7W<'_, LpStore7Spec> {
        LpScratch7W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpStore7Spec;
impl crate::RegisterSpec for LpStore7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store7::R`](R) reader structure"]
impl crate::Readable for LpStore7Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_store7::W`](W) writer structure"]
impl crate::Writable for LpStore7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_STORE7 to value 0"]
impl crate::Resettable for LpStore7Spec {}

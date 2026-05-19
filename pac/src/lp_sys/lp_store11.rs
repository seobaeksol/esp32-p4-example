#[doc = "Register `LP_STORE11` reader"]
pub type R = crate::R<LpStore11Spec>;
#[doc = "Register `LP_STORE11` writer"]
pub type W = crate::W<LpStore11Spec>;
#[doc = "Field `LP_SCRATCH11` reader - need_des"]
pub type LpScratch11R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH11` writer - need_des"]
pub type LpScratch11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch11(&self) -> LpScratch11R {
        LpScratch11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch11(&mut self) -> LpScratch11W<'_, LpStore11Spec> {
        LpScratch11W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpStore11Spec;
impl crate::RegisterSpec for LpStore11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store11::R`](R) reader structure"]
impl crate::Readable for LpStore11Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_store11::W`](W) writer structure"]
impl crate::Writable for LpStore11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_STORE11 to value 0"]
impl crate::Resettable for LpStore11Spec {}

#[doc = "Register `LP_STORE9` reader"]
pub type R = crate::R<LpStore9Spec>;
#[doc = "Register `LP_STORE9` writer"]
pub type W = crate::W<LpStore9Spec>;
#[doc = "Field `LP_SCRATCH9` reader - need_des"]
pub type LpScratch9R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH9` writer - need_des"]
pub type LpScratch9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch9(&self) -> LpScratch9R {
        LpScratch9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch9(&mut self) -> LpScratch9W<'_, LpStore9Spec> {
        LpScratch9W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpStore9Spec;
impl crate::RegisterSpec for LpStore9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store9::R`](R) reader structure"]
impl crate::Readable for LpStore9Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_store9::W`](W) writer structure"]
impl crate::Writable for LpStore9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_STORE9 to value 0"]
impl crate::Resettable for LpStore9Spec {}

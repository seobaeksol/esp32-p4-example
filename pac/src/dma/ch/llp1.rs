#[doc = "Register `LLP1` reader"]
pub type R = crate::R<Llp1Spec>;
#[doc = "Register `LLP1` writer"]
pub type W = crate::W<Llp1Spec>;
#[doc = "Field `CH1_LOC1` reader - NA"]
pub type Ch1Loc1R = crate::FieldReader<u32>;
#[doc = "Field `CH1_LOC1` writer - NA"]
pub type Ch1Loc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_loc1(&self) -> Ch1Loc1R {
        Ch1Loc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_loc1(&mut self) -> Ch1Loc1W<'_, Llp1Spec> {
        Ch1Loc1W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`llp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp1Spec;
impl crate::RegisterSpec for Llp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp1::R`](R) reader structure"]
impl crate::Readable for Llp1Spec {}
#[doc = "`write(|w| ..)` method takes [`llp1::W`](W) writer structure"]
impl crate::Writable for Llp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP1 to value 0"]
impl crate::Resettable for Llp1Spec {}

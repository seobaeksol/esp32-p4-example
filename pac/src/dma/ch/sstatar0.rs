#[doc = "Register `SSTATAR0` reader"]
pub type R = crate::R<Sstatar0Spec>;
#[doc = "Register `SSTATAR0` writer"]
pub type W = crate::W<Sstatar0Spec>;
#[doc = "Field `CH1_SSTATAR0` reader - NA"]
pub type Ch1Sstatar0R = crate::FieldReader<u32>;
#[doc = "Field `CH1_SSTATAR0` writer - NA"]
pub type Ch1Sstatar0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstatar0(&self) -> Ch1Sstatar0R {
        Ch1Sstatar0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstatar0(&mut self) -> Ch1Sstatar0W<'_, Sstatar0Spec> {
        Ch1Sstatar0W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstatar0Spec;
impl crate::RegisterSpec for Sstatar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar0::R`](R) reader structure"]
impl crate::Readable for Sstatar0Spec {}
#[doc = "`write(|w| ..)` method takes [`sstatar0::W`](W) writer structure"]
impl crate::Writable for Sstatar0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTATAR0 to value 0"]
impl crate::Resettable for Sstatar0Spec {}

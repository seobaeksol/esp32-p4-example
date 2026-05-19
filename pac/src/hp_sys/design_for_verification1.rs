#[doc = "Register `DESIGN_FOR_VERIFICATION1` reader"]
pub type R = crate::R<DesignForVerification1Spec>;
#[doc = "Register `DESIGN_FOR_VERIFICATION1` writer"]
pub type W = crate::W<DesignForVerification1Spec>;
#[doc = "Field `DFV1` reader - register for DV"]
pub type Dfv1R = crate::FieldReader<u32>;
#[doc = "Field `DFV1` writer - register for DV"]
pub type Dfv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    pub fn dfv1(&self) -> Dfv1R {
        Dfv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    pub fn dfv1(&mut self) -> Dfv1W<'_, DesignForVerification1Spec> {
        Dfv1W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`design_for_verification1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`design_for_verification1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DesignForVerification1Spec;
impl crate::RegisterSpec for DesignForVerification1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`design_for_verification1::R`](R) reader structure"]
impl crate::Readable for DesignForVerification1Spec {}
#[doc = "`write(|w| ..)` method takes [`design_for_verification1::W`](W) writer structure"]
impl crate::Writable for DesignForVerification1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DESIGN_FOR_VERIFICATION1 to value 0"]
impl crate::Resettable for DesignForVerification1Spec {}

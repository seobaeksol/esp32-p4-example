#[doc = "Register `TOUCH_SLP1` reader"]
pub type R = crate::R<TouchSlp1Spec>;
#[doc = "Register `TOUCH_SLP1` writer"]
pub type W = crate::W<TouchSlp1Spec>;
#[doc = "Field `TOUCH_SLP_TH2` reader - need_des"]
pub type TouchSlpTh2R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLP_TH2` writer - need_des"]
pub type TouchSlpTh2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_SLP_TH1` reader - need_des"]
pub type TouchSlpTh1R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLP_TH1` writer - need_des"]
pub type TouchSlpTh1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th2(&self) -> TouchSlpTh2R {
        TouchSlpTh2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th1(&self) -> TouchSlpTh1R {
        TouchSlpTh1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th2(&mut self) -> TouchSlpTh2W<'_, TouchSlp1Spec> {
        TouchSlpTh2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th1(&mut self) -> TouchSlpTh1W<'_, TouchSlp1Spec> {
        TouchSlpTh1W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_slp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_slp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchSlp1Spec;
impl crate::RegisterSpec for TouchSlp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_slp1::R`](R) reader structure"]
impl crate::Readable for TouchSlp1Spec {}
#[doc = "`write(|w| ..)` method takes [`touch_slp1::W`](W) writer structure"]
impl crate::Writable for TouchSlp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_SLP1 to value 0"]
impl crate::Resettable for TouchSlp1Spec {}

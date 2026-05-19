#[doc = "Register `TOUCH_PAD10_TH1` reader"]
pub type R = crate::R<TouchPad10Th1Spec>;
#[doc = "Register `TOUCH_PAD10_TH1` writer"]
pub type W = crate::W<TouchPad10Th1Spec>;
#[doc = "Field `TOUCH_PAD10_TH1` reader - Reserved"]
pub type TouchPad10Th1R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD10_TH1` writer - Reserved"]
pub type TouchPad10Th1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad10_th1(&self) -> TouchPad10Th1R {
        TouchPad10Th1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad10_th1(&mut self) -> TouchPad10Th1W<'_, TouchPad10Th1Spec> {
        TouchPad10Th1W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pad10_th1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pad10_th1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchPad10Th1Spec;
impl crate::RegisterSpec for TouchPad10Th1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pad10_th1::R`](R) reader structure"]
impl crate::Readable for TouchPad10Th1Spec {}
#[doc = "`write(|w| ..)` method takes [`touch_pad10_th1::W`](W) writer structure"]
impl crate::Writable for TouchPad10Th1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_PAD10_TH1 to value 0"]
impl crate::Resettable for TouchPad10Th1Spec {}

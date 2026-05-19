#[doc = "Register `TOUCH_PAD1_TH0` reader"]
pub type R = crate::R<TouchPad1Th0Spec>;
#[doc = "Register `TOUCH_PAD1_TH0` writer"]
pub type W = crate::W<TouchPad1Th0Spec>;
#[doc = "Field `TOUCH_PAD1_TH0` reader - Reserved"]
pub type TouchPad1Th0R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD1_TH0` writer - Reserved"]
pub type TouchPad1Th0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad1_th0(&self) -> TouchPad1Th0R {
        TouchPad1Th0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad1_th0(&mut self) -> TouchPad1Th0W<'_, TouchPad1Th0Spec> {
        TouchPad1Th0W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pad1_th0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pad1_th0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchPad1Th0Spec;
impl crate::RegisterSpec for TouchPad1Th0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pad1_th0::R`](R) reader structure"]
impl crate::Readable for TouchPad1Th0Spec {}
#[doc = "`write(|w| ..)` method takes [`touch_pad1_th0::W`](W) writer structure"]
impl crate::Writable for TouchPad1Th0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_PAD1_TH0 to value 0"]
impl crate::Resettable for TouchPad1Th0Spec {}

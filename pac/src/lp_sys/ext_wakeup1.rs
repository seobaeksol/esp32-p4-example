#[doc = "Register `EXT_WAKEUP1` reader"]
pub type R = crate::R<ExtWakeup1Spec>;
#[doc = "Register `EXT_WAKEUP1` writer"]
pub type W = crate::W<ExtWakeup1Spec>;
#[doc = "Field `SEL` reader - Bitmap to select RTC pads for ext wakeup1"]
pub type SelR = crate::FieldReader<u16>;
#[doc = "Field `SEL` writer - Bitmap to select RTC pads for ext wakeup1"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STATUS_CLR` writer - clear ext wakeup1 status"]
pub type StatusClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, ExtWakeup1Spec> {
        SelW::new(self, 0)
    }
    #[doc = "Bit 16 - clear ext wakeup1 status"]
    #[inline(always)]
    pub fn status_clr(&mut self) -> StatusClrW<'_, ExtWakeup1Spec> {
        StatusClrW::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtWakeup1Spec;
impl crate::RegisterSpec for ExtWakeup1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup1::R`](R) reader structure"]
impl crate::Readable for ExtWakeup1Spec {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup1::W`](W) writer structure"]
impl crate::Writable for ExtWakeup1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_WAKEUP1 to value 0"]
impl crate::Resettable for ExtWakeup1Spec {}

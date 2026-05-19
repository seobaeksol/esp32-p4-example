#[doc = "Register `TOUCH_FILTER3` reader"]
pub type R = crate::R<TouchFilter3Spec>;
#[doc = "Register `TOUCH_FILTER3` writer"]
pub type W = crate::W<TouchFilter3Spec>;
#[doc = "Field `TOUCH_BASELINE_SW` reader - need_des"]
pub type TouchBaselineSwR = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_BASELINE_SW` writer - need_des"]
pub type TouchBaselineSwW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_UPDATE_BASELINE_SW` writer - need_des"]
pub type TouchUpdateBaselineSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_baseline_sw(&self) -> TouchBaselineSwR {
        TouchBaselineSwR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_baseline_sw(&mut self) -> TouchBaselineSwW<'_, TouchFilter3Spec> {
        TouchBaselineSwW::new(self, 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn touch_update_baseline_sw(&mut self) -> TouchUpdateBaselineSwW<'_, TouchFilter3Spec> {
        TouchUpdateBaselineSwW::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_filter3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_filter3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchFilter3Spec;
impl crate::RegisterSpec for TouchFilter3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_filter3::R`](R) reader structure"]
impl crate::Readable for TouchFilter3Spec {}
#[doc = "`write(|w| ..)` method takes [`touch_filter3::W`](W) writer structure"]
impl crate::Writable for TouchFilter3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_FILTER3 to value 0"]
impl crate::Resettable for TouchFilter3Spec {}

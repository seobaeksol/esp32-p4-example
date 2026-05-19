#[doc = "Register `SHARP_FILTER2` reader"]
pub type R = crate::R<SharpFilter2Spec>;
#[doc = "Register `SHARP_FILTER2` writer"]
pub type W = crate::W<SharpFilter2Spec>;
#[doc = "Field `SHARP_FILTER_COE20` reader - this field configures usm filter coefficient"]
pub type SharpFilterCoe20R = crate::FieldReader;
#[doc = "Field `SHARP_FILTER_COE20` writer - this field configures usm filter coefficient"]
pub type SharpFilterCoe20W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SHARP_FILTER_COE21` reader - this field configures usm filter coefficient"]
pub type SharpFilterCoe21R = crate::FieldReader;
#[doc = "Field `SHARP_FILTER_COE21` writer - this field configures usm filter coefficient"]
pub type SharpFilterCoe21W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SHARP_FILTER_COE22` reader - this field configures usm filter coefficient"]
pub type SharpFilterCoe22R = crate::FieldReader;
#[doc = "Field `SHARP_FILTER_COE22` writer - this field configures usm filter coefficient"]
pub type SharpFilterCoe22W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe20(&self) -> SharpFilterCoe20R {
        SharpFilterCoe20R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe21(&self) -> SharpFilterCoe21R {
        SharpFilterCoe21R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe22(&self) -> SharpFilterCoe22R {
        SharpFilterCoe22R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe20(&mut self) -> SharpFilterCoe20W<'_, SharpFilter2Spec> {
        SharpFilterCoe20W::new(self, 0)
    }
    #[doc = "Bits 5:9 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe21(&mut self) -> SharpFilterCoe21W<'_, SharpFilter2Spec> {
        SharpFilterCoe21W::new(self, 5)
    }
    #[doc = "Bits 10:14 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe22(&mut self) -> SharpFilterCoe22W<'_, SharpFilter2Spec> {
        SharpFilterCoe22W::new(self, 10)
    }
}
#[doc = "sharp usm config register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_filter2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_filter2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharpFilter2Spec;
impl crate::RegisterSpec for SharpFilter2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharp_filter2::R`](R) reader structure"]
impl crate::Readable for SharpFilter2Spec {}
#[doc = "`write(|w| ..)` method takes [`sharp_filter2::W`](W) writer structure"]
impl crate::Writable for SharpFilter2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHARP_FILTER2 to value 0x0441"]
impl crate::Resettable for SharpFilter2Spec {
    const RESET_VALUE: u32 = 0x0441;
}

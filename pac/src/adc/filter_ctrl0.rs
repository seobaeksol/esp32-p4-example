#[doc = "Register `FILTER_CTRL0` reader"]
pub type R = crate::R<FilterCtrl0Spec>;
#[doc = "Register `FILTER_CTRL0` writer"]
pub type W = crate::W<FilterCtrl0Spec>;
#[doc = "Field `FILTER_CHANNEL1` reader - need_des"]
pub type FilterChannel1R = crate::FieldReader;
#[doc = "Field `FILTER_CHANNEL1` writer - need_des"]
pub type FilterChannel1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FILTER_CHANNEL0` reader - apb_adc1_filter_factor"]
pub type FilterChannel0R = crate::FieldReader;
#[doc = "Field `FILTER_CHANNEL0` writer - apb_adc1_filter_factor"]
pub type FilterChannel0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FILTER_RESET` reader - enable apb_adc1_filter"]
pub type FilterResetR = crate::BitReader;
#[doc = "Field `FILTER_RESET` writer - enable apb_adc1_filter"]
pub type FilterResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 14:18 - need_des"]
    #[inline(always)]
    pub fn filter_channel1(&self) -> FilterChannel1R {
        FilterChannel1R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - apb_adc1_filter_factor"]
    #[inline(always)]
    pub fn filter_channel0(&self) -> FilterChannel0R {
        FilterChannel0R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    pub fn filter_reset(&self) -> FilterResetR {
        FilterResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 14:18 - need_des"]
    #[inline(always)]
    pub fn filter_channel1(&mut self) -> FilterChannel1W<'_, FilterCtrl0Spec> {
        FilterChannel1W::new(self, 14)
    }
    #[doc = "Bits 19:23 - apb_adc1_filter_factor"]
    #[inline(always)]
    pub fn filter_channel0(&mut self) -> FilterChannel0W<'_, FilterCtrl0Spec> {
        FilterChannel0W::new(self, 19)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    pub fn filter_reset(&mut self) -> FilterResetW<'_, FilterCtrl0Spec> {
        FilterResetW::new(self, 31)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterCtrl0Spec;
impl crate::RegisterSpec for FilterCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_ctrl0::R`](R) reader structure"]
impl crate::Readable for FilterCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`filter_ctrl0::W`](W) writer structure"]
impl crate::Writable for FilterCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_CTRL0 to value 0x006b_4000"]
impl crate::Resettable for FilterCtrl0Spec {
    const RESET_VALUE: u32 = 0x006b_4000;
}

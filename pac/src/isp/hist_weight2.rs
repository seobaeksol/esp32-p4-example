#[doc = "Register `HIST_WEIGHT2` reader"]
pub type R = crate::R<HistWeight2Spec>;
#[doc = "Register `HIST_WEIGHT2` writer"]
pub type W = crate::W<HistWeight2Spec>;
#[doc = "Field `HIST_WEIGHT_21` reader - this field configures weight of subwindow 21"]
pub type HistWeight21R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_21` writer - this field configures weight of subwindow 21"]
pub type HistWeight21W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_20` reader - this field configures weight of subwindow 20"]
pub type HistWeight20R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_20` writer - this field configures weight of subwindow 20"]
pub type HistWeight20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_14` reader - this field configures weight of subwindow 04"]
pub type HistWeight14R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_14` writer - this field configures weight of subwindow 04"]
pub type HistWeight14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_13` reader - this field configures weight of subwindow 13"]
pub type HistWeight13R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_13` writer - this field configures weight of subwindow 13"]
pub type HistWeight13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 21"]
    #[inline(always)]
    pub fn hist_weight_21(&self) -> HistWeight21R {
        HistWeight21R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 20"]
    #[inline(always)]
    pub fn hist_weight_20(&self) -> HistWeight20R {
        HistWeight20R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 04"]
    #[inline(always)]
    pub fn hist_weight_14(&self) -> HistWeight14R {
        HistWeight14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 13"]
    #[inline(always)]
    pub fn hist_weight_13(&self) -> HistWeight13R {
        HistWeight13R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 21"]
    #[inline(always)]
    pub fn hist_weight_21(&mut self) -> HistWeight21W<'_, HistWeight2Spec> {
        HistWeight21W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 20"]
    #[inline(always)]
    pub fn hist_weight_20(&mut self) -> HistWeight20W<'_, HistWeight2Spec> {
        HistWeight20W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 04"]
    #[inline(always)]
    pub fn hist_weight_14(&mut self) -> HistWeight14W<'_, HistWeight2Spec> {
        HistWeight14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 13"]
    #[inline(always)]
    pub fn hist_weight_13(&mut self) -> HistWeight13W<'_, HistWeight2Spec> {
        HistWeight13W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight2Spec;
impl crate::RegisterSpec for HistWeight2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight2::R`](R) reader structure"]
impl crate::Readable for HistWeight2Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight2::W`](W) writer structure"]
impl crate::Writable for HistWeight2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_WEIGHT2 to value 0x0101_0101"]
impl crate::Resettable for HistWeight2Spec {
    const RESET_VALUE: u32 = 0x0101_0101;
}

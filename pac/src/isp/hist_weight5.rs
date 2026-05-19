#[doc = "Register `HIST_WEIGHT5` reader"]
pub type R = crate::R<HistWeight5Spec>;
#[doc = "Register `HIST_WEIGHT5` writer"]
pub type W = crate::W<HistWeight5Spec>;
#[doc = "Field `HIST_WEIGHT_43` reader - this field configures weight of subwindow 43"]
pub type HistWeight43R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_43` writer - this field configures weight of subwindow 43"]
pub type HistWeight43W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_42` reader - this field configures weight of subwindow 42"]
pub type HistWeight42R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_42` writer - this field configures weight of subwindow 42"]
pub type HistWeight42W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_41` reader - this field configures weight of subwindow 41"]
pub type HistWeight41R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_41` writer - this field configures weight of subwindow 41"]
pub type HistWeight41W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_40` reader - this field configures weight of subwindow 40"]
pub type HistWeight40R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_40` writer - this field configures weight of subwindow 40"]
pub type HistWeight40W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 43"]
    #[inline(always)]
    pub fn hist_weight_43(&self) -> HistWeight43R {
        HistWeight43R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 42"]
    #[inline(always)]
    pub fn hist_weight_42(&self) -> HistWeight42R {
        HistWeight42R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 41"]
    #[inline(always)]
    pub fn hist_weight_41(&self) -> HistWeight41R {
        HistWeight41R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 40"]
    #[inline(always)]
    pub fn hist_weight_40(&self) -> HistWeight40R {
        HistWeight40R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 43"]
    #[inline(always)]
    pub fn hist_weight_43(&mut self) -> HistWeight43W<'_, HistWeight5Spec> {
        HistWeight43W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 42"]
    #[inline(always)]
    pub fn hist_weight_42(&mut self) -> HistWeight42W<'_, HistWeight5Spec> {
        HistWeight42W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 41"]
    #[inline(always)]
    pub fn hist_weight_41(&mut self) -> HistWeight41W<'_, HistWeight5Spec> {
        HistWeight41W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 40"]
    #[inline(always)]
    pub fn hist_weight_40(&mut self) -> HistWeight40W<'_, HistWeight5Spec> {
        HistWeight40W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight5Spec;
impl crate::RegisterSpec for HistWeight5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight5::R`](R) reader structure"]
impl crate::Readable for HistWeight5Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight5::W`](W) writer structure"]
impl crate::Writable for HistWeight5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_WEIGHT5 to value 0x0101_0101"]
impl crate::Resettable for HistWeight5Spec {
    const RESET_VALUE: u32 = 0x0101_0101;
}

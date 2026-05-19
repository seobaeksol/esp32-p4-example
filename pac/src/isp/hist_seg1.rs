#[doc = "Register `HIST_SEG1` reader"]
pub type R = crate::R<HistSeg1Spec>;
#[doc = "Register `HIST_SEG1` writer"]
pub type W = crate::W<HistSeg1Spec>;
#[doc = "Field `HIST_SEG_7_8` reader - this field configures threshold of histogram bin 7 and bin 8"]
pub type HistSeg7_8R = crate::FieldReader;
#[doc = "Field `HIST_SEG_7_8` writer - this field configures threshold of histogram bin 7 and bin 8"]
pub type HistSeg7_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_6_7` reader - this field configures threshold of histogram bin 6 and bin 7"]
pub type HistSeg6_7R = crate::FieldReader;
#[doc = "Field `HIST_SEG_6_7` writer - this field configures threshold of histogram bin 6 and bin 7"]
pub type HistSeg6_7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_5_6` reader - this field configures threshold of histogram bin 5 and bin 6"]
pub type HistSeg5_6R = crate::FieldReader;
#[doc = "Field `HIST_SEG_5_6` writer - this field configures threshold of histogram bin 5 and bin 6"]
pub type HistSeg5_6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_4_5` reader - this field configures threshold of histogram bin 4 and bin 5"]
pub type HistSeg4_5R = crate::FieldReader;
#[doc = "Field `HIST_SEG_4_5` writer - this field configures threshold of histogram bin 4 and bin 5"]
pub type HistSeg4_5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 7 and bin 8"]
    #[inline(always)]
    pub fn hist_seg_7_8(&self) -> HistSeg7_8R {
        HistSeg7_8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 6 and bin 7"]
    #[inline(always)]
    pub fn hist_seg_6_7(&self) -> HistSeg6_7R {
        HistSeg6_7R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 5 and bin 6"]
    #[inline(always)]
    pub fn hist_seg_5_6(&self) -> HistSeg5_6R {
        HistSeg5_6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 4 and bin 5"]
    #[inline(always)]
    pub fn hist_seg_4_5(&self) -> HistSeg4_5R {
        HistSeg4_5R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 7 and bin 8"]
    #[inline(always)]
    pub fn hist_seg_7_8(&mut self) -> HistSeg7_8W<'_, HistSeg1Spec> {
        HistSeg7_8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 6 and bin 7"]
    #[inline(always)]
    pub fn hist_seg_6_7(&mut self) -> HistSeg6_7W<'_, HistSeg1Spec> {
        HistSeg6_7W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 5 and bin 6"]
    #[inline(always)]
    pub fn hist_seg_5_6(&mut self) -> HistSeg5_6W<'_, HistSeg1Spec> {
        HistSeg5_6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 4 and bin 5"]
    #[inline(always)]
    pub fn hist_seg_4_5(&mut self) -> HistSeg4_5W<'_, HistSeg1Spec> {
        HistSeg4_5W::new(self, 24)
    }
}
#[doc = "histogram bin control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_seg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_seg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistSeg1Spec;
impl crate::RegisterSpec for HistSeg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_seg1::R`](R) reader structure"]
impl crate::Readable for HistSeg1Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_seg1::W`](W) writer structure"]
impl crate::Writable for HistSeg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_SEG1 to value 0x5060_7080"]
impl crate::Resettable for HistSeg1Spec {
    const RESET_VALUE: u32 = 0x5060_7080;
}

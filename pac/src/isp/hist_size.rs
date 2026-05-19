#[doc = "Register `HIST_SIZE` reader"]
pub type R = crate::R<HistSizeSpec>;
#[doc = "Register `HIST_SIZE` writer"]
pub type W = crate::W<HistSizeSpec>;
#[doc = "Field `HIST_Y_SIZE` reader - this field configures y direction size of subwindow"]
pub type HistYSizeR = crate::FieldReader<u16>;
#[doc = "Field `HIST_Y_SIZE` writer - this field configures y direction size of subwindow"]
pub type HistYSizeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `HIST_X_SIZE` reader - this field configures x direction size of subwindow"]
pub type HistXSizeR = crate::FieldReader<u16>;
#[doc = "Field `HIST_X_SIZE` writer - this field configures x direction size of subwindow"]
pub type HistXSizeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - this field configures y direction size of subwindow"]
    #[inline(always)]
    pub fn hist_y_size(&self) -> HistYSizeR {
        HistYSizeR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - this field configures x direction size of subwindow"]
    #[inline(always)]
    pub fn hist_x_size(&self) -> HistXSizeR {
        HistXSizeR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - this field configures y direction size of subwindow"]
    #[inline(always)]
    pub fn hist_y_size(&mut self) -> HistYSizeW<'_, HistSizeSpec> {
        HistYSizeW::new(self, 0)
    }
    #[doc = "Bits 16:24 - this field configures x direction size of subwindow"]
    #[inline(always)]
    pub fn hist_x_size(&mut self) -> HistXSizeW<'_, HistSizeSpec> {
        HistXSizeW::new(self, 16)
    }
}
#[doc = "histogram sub-window size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistSizeSpec;
impl crate::RegisterSpec for HistSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_size::R`](R) reader structure"]
impl crate::Readable for HistSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_size::W`](W) writer structure"]
impl crate::Writable for HistSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_SIZE to value 0x0012_0020"]
impl crate::Resettable for HistSizeSpec {
    const RESET_VALUE: u32 = 0x0012_0020;
}

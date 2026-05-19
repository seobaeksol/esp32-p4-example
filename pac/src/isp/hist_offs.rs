#[doc = "Register `HIST_OFFS` reader"]
pub type R = crate::R<HistOffsSpec>;
#[doc = "Register `HIST_OFFS` writer"]
pub type W = crate::W<HistOffsSpec>;
#[doc = "Field `HIST_Y_OFFS` reader - this field configures y coordinate of first window"]
pub type HistYOffsR = crate::FieldReader<u16>;
#[doc = "Field `HIST_Y_OFFS` writer - this field configures y coordinate of first window"]
pub type HistYOffsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HIST_X_OFFS` reader - this field configures x coordinate of first window"]
pub type HistXOffsR = crate::FieldReader<u16>;
#[doc = "Field `HIST_X_OFFS` writer - this field configures x coordinate of first window"]
pub type HistXOffsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures y coordinate of first window"]
    #[inline(always)]
    pub fn hist_y_offs(&self) -> HistYOffsR {
        HistYOffsR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures x coordinate of first window"]
    #[inline(always)]
    pub fn hist_x_offs(&self) -> HistXOffsR {
        HistXOffsR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures y coordinate of first window"]
    #[inline(always)]
    pub fn hist_y_offs(&mut self) -> HistYOffsW<'_, HistOffsSpec> {
        HistYOffsW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures x coordinate of first window"]
    #[inline(always)]
    pub fn hist_x_offs(&mut self) -> HistXOffsW<'_, HistOffsSpec> {
        HistXOffsW::new(self, 16)
    }
}
#[doc = "histogram window offsets register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_offs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_offs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistOffsSpec;
impl crate::RegisterSpec for HistOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_offs::R`](R) reader structure"]
impl crate::Readable for HistOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_offs::W`](W) writer structure"]
impl crate::Writable for HistOffsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_OFFS to value 0"]
impl crate::Resettable for HistOffsSpec {}

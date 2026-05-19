#[doc = "Register `HIST_MODE` reader"]
pub type R = crate::R<HistModeSpec>;
#[doc = "Register `HIST_MODE` writer"]
pub type W = crate::W<HistModeSpec>;
#[doc = "Field `HIST_MODE` reader - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V"]
pub type HistModeR = crate::FieldReader;
#[doc = "Field `HIST_MODE` writer - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V"]
pub type HistModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V"]
    #[inline(always)]
    pub fn hist_mode(&self) -> HistModeR {
        HistModeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V"]
    #[inline(always)]
    pub fn hist_mode(&mut self) -> HistModeW<'_, HistModeSpec> {
        HistModeW::new(self, 0)
    }
}
#[doc = "histogram mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistModeSpec;
impl crate::RegisterSpec for HistModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_mode::R`](R) reader structure"]
impl crate::Readable for HistModeSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_mode::W`](W) writer structure"]
impl crate::Writable for HistModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_MODE to value 0x04"]
impl crate::Resettable for HistModeSpec {
    const RESET_VALUE: u32 = 0x04;
}

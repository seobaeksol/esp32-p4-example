#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `THRES1_LOW` reader - need_des"]
pub type Thres1LowR = crate::BitReader;
#[doc = "Field `THRES1_LOW` writer - need_des"]
pub type Thres1LowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_LOW` reader - need_des"]
pub type Thres0LowR = crate::BitReader;
#[doc = "Field `THRES0_LOW` writer - need_des"]
pub type Thres0LowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES1_HIGH` reader - need_des"]
pub type Thres1HighR = crate::BitReader;
#[doc = "Field `THRES1_HIGH` writer - need_des"]
pub type Thres1HighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_HIGH` reader - need_des"]
pub type Thres0HighR = crate::BitReader;
#[doc = "Field `THRES0_HIGH` writer - need_des"]
pub type Thres0HighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_DONE` reader - need_des"]
pub type Adc2DoneR = crate::BitReader;
#[doc = "Field `ADC2_DONE` writer - need_des"]
pub type Adc2DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_DONE` reader - need_des"]
pub type Adc1DoneR = crate::BitReader;
#[doc = "Field `ADC1_DONE` writer - need_des"]
pub type Adc1DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn thres1_low(&self) -> Thres1LowR {
        Thres1LowR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres0_low(&self) -> Thres0LowR {
        Thres0LowR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres1_high(&self) -> Thres1HighR {
        Thres1HighR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres0_high(&self) -> Thres0HighR {
        Thres0HighR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn adc2_done(&self) -> Adc2DoneR {
        Adc2DoneR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn adc1_done(&self) -> Adc1DoneR {
        Adc1DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn thres1_low(&mut self) -> Thres1LowW<'_, IntEnaSpec> {
        Thres1LowW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres0_low(&mut self) -> Thres0LowW<'_, IntEnaSpec> {
        Thres0LowW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres1_high(&mut self) -> Thres1HighW<'_, IntEnaSpec> {
        Thres1HighW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres0_high(&mut self) -> Thres0HighW<'_, IntEnaSpec> {
        Thres0HighW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn adc2_done(&mut self) -> Adc2DoneW<'_, IntEnaSpec> {
        Adc2DoneW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn adc1_done(&mut self) -> Adc1DoneW<'_, IntEnaSpec> {
        Adc1DoneW::new(self, 31)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}

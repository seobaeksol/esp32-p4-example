#[doc = "Register `SCL_LOW_PERIOD` reader"]
pub type R = crate::R<SclLowPeriodSpec>;
#[doc = "Register `SCL_LOW_PERIOD` writer"]
pub type W = crate::W<SclLowPeriodSpec>;
#[doc = "Field `SCL_LOW_PERIOD` reader - Configures the low level width of the SCL Clock in master mode. \\\\ Measurement unit: i2c_sclk \\\\"]
pub type SclLowPeriodR = crate::FieldReader<u16>;
#[doc = "Field `SCL_LOW_PERIOD` writer - Configures the low level width of the SCL Clock in master mode. \\\\ Measurement unit: i2c_sclk \\\\"]
pub type SclLowPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Configures the low level width of the SCL Clock in master mode. \\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn scl_low_period(&self) -> SclLowPeriodR {
        SclLowPeriodR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Configures the low level width of the SCL Clock in master mode. \\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn scl_low_period(&mut self) -> SclLowPeriodW<'_, SclLowPeriodSpec> {
        SclLowPeriodW::new(self, 0)
    }
}
#[doc = "Configures the low level width of the SCL Clock\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_low_period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_low_period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclLowPeriodSpec;
impl crate::RegisterSpec for SclLowPeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_low_period::R`](R) reader structure"]
impl crate::Readable for SclLowPeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_low_period::W`](W) writer structure"]
impl crate::Writable for SclLowPeriodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_LOW_PERIOD to value 0"]
impl crate::Resettable for SclLowPeriodSpec {}

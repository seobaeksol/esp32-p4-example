#[doc = "Register `SCL_I2C_FMP_TIME` reader"]
pub type R = crate::R<SclI2cFmpTimeSpec>;
#[doc = "Register `SCL_I2C_FMP_TIME` writer"]
pub type W = crate::W<SclI2cFmpTimeSpec>;
#[doc = "Field `REG_I2C_FMP_LOW_PERIOD` reader - NA"]
pub type RegI2cFmpLowPeriodR = crate::FieldReader<u16>;
#[doc = "Field `REG_I2C_FMP_LOW_PERIOD` writer - NA"]
pub type RegI2cFmpLowPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_I2C_FMP_HIGH_PERIOD` reader - NA"]
pub type RegI2cFmpHighPeriodR = crate::FieldReader;
#[doc = "Field `REG_I2C_FMP_HIGH_PERIOD` writer - NA"]
pub type RegI2cFmpHighPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_low_period(&self) -> RegI2cFmpLowPeriodR {
        RegI2cFmpLowPeriodR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_high_period(&self) -> RegI2cFmpHighPeriodR {
        RegI2cFmpHighPeriodR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_low_period(&mut self) -> RegI2cFmpLowPeriodW<'_, SclI2cFmpTimeSpec> {
        RegI2cFmpLowPeriodW::new(self, 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_high_period(&mut self) -> RegI2cFmpHighPeriodW<'_, SclI2cFmpTimeSpec> {
        RegI2cFmpHighPeriodW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_i2c_fmp_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_i2c_fmp_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclI2cFmpTimeSpec;
impl crate::RegisterSpec for SclI2cFmpTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_i2c_fmp_time::R`](R) reader structure"]
impl crate::Readable for SclI2cFmpTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_i2c_fmp_time::W`](W) writer structure"]
impl crate::Writable for SclI2cFmpTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_I2C_FMP_TIME to value 0x0021_003f"]
impl crate::Resettable for SclI2cFmpTimeSpec {
    const RESET_VALUE: u32 = 0x0021_003f;
}

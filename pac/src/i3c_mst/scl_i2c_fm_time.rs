#[doc = "Register `SCL_I2C_FM_TIME` reader"]
pub type R = crate::R<SclI2cFmTimeSpec>;
#[doc = "Register `SCL_I2C_FM_TIME` writer"]
pub type W = crate::W<SclI2cFmTimeSpec>;
#[doc = "Field `REG_I2C_FM_LOW_PERIOD` reader - NA"]
pub type RegI2cFmLowPeriodR = crate::FieldReader<u16>;
#[doc = "Field `REG_I2C_FM_LOW_PERIOD` writer - NA"]
pub type RegI2cFmLowPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_I2C_FM_HIGH_PERIOD` reader - The SCL open-drain low count timing for I2C Fast Mode transfers."]
pub type RegI2cFmHighPeriodR = crate::FieldReader<u16>;
#[doc = "Field `REG_I2C_FM_HIGH_PERIOD` writer - The SCL open-drain low count timing for I2C Fast Mode transfers."]
pub type RegI2cFmHighPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fm_low_period(&self) -> RegI2cFmLowPeriodR {
        RegI2cFmLowPeriodR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The SCL open-drain low count timing for I2C Fast Mode transfers."]
    #[inline(always)]
    pub fn reg_i2c_fm_high_period(&self) -> RegI2cFmHighPeriodR {
        RegI2cFmHighPeriodR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fm_low_period(&mut self) -> RegI2cFmLowPeriodW<'_, SclI2cFmTimeSpec> {
        RegI2cFmLowPeriodW::new(self, 0)
    }
    #[doc = "Bits 16:31 - The SCL open-drain low count timing for I2C Fast Mode transfers."]
    #[inline(always)]
    pub fn reg_i2c_fm_high_period(&mut self) -> RegI2cFmHighPeriodW<'_, SclI2cFmTimeSpec> {
        RegI2cFmHighPeriodW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_i2c_fm_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_i2c_fm_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclI2cFmTimeSpec;
impl crate::RegisterSpec for SclI2cFmTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_i2c_fm_time::R`](R) reader structure"]
impl crate::Readable for SclI2cFmTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_i2c_fm_time::W`](W) writer structure"]
impl crate::Writable for SclI2cFmTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_I2C_FM_TIME to value 0x004b_00a3"]
impl crate::Resettable for SclI2cFmTimeSpec {
    const RESET_VALUE: u32 = 0x004b_00a3;
}

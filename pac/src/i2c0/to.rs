#[doc = "Register `TO` reader"]
pub type R = crate::R<ToSpec>;
#[doc = "Register `TO` writer"]
pub type W = crate::W<ToSpec>;
#[doc = "Field `TIME_OUT_VALUE` reader - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2\\^{}(reg_time_out_value).\\\\ Measurement unit: i2c_sclk \\\\"]
pub type TimeOutValueR = crate::FieldReader;
#[doc = "Field `TIME_OUT_VALUE` writer - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2\\^{}(reg_time_out_value).\\\\ Measurement unit: i2c_sclk \\\\"]
pub type TimeOutValueW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TIME_OUT_EN` reader - Configures to enable time out control.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type TimeOutEnR = crate::BitReader;
#[doc = "Field `TIME_OUT_EN` writer - Configures to enable time out control.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type TimeOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2\\^{}(reg_time_out_value).\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn time_out_value(&self) -> TimeOutValueR {
        TimeOutValueR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Configures to enable time out control.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn time_out_en(&self) -> TimeOutEnR {
        TimeOutEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2\\^{}(reg_time_out_value).\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn time_out_value(&mut self) -> TimeOutValueW<'_, ToSpec> {
        TimeOutValueW::new(self, 0)
    }
    #[doc = "Bit 5 - Configures to enable time out control.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn time_out_en(&mut self) -> TimeOutEnW<'_, ToSpec> {
        TimeOutEnW::new(self, 5)
    }
}
#[doc = "Setting time out control for receiving data\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToSpec;
impl crate::RegisterSpec for ToSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to::R`](R) reader structure"]
impl crate::Readable for ToSpec {}
#[doc = "`write(|w| ..)` method takes [`to::W`](W) writer structure"]
impl crate::Writable for ToSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TO to value 0x10"]
impl crate::Resettable for ToSpec {
    const RESET_VALUE: u32 = 0x10;
}

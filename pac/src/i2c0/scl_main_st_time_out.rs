#[doc = "Register `SCL_MAIN_ST_TIME_OUT` reader"]
pub type R = crate::R<SclMainStTimeOutSpec>;
#[doc = "Register `SCL_MAIN_ST_TIME_OUT` writer"]
pub type W = crate::W<SclMainStTimeOutSpec>;
#[doc = "Field `SCL_MAIN_ST_TO` reader - Configures the threshold value of SCL_MAIN_FSM state unchanged period. It should be no more than 23.\\\\ Measurement unit: i2c_sclk \\\\"]
pub type SclMainStToR = crate::FieldReader;
#[doc = "Field `SCL_MAIN_ST_TO` writer - Configures the threshold value of SCL_MAIN_FSM state unchanged period. It should be no more than 23.\\\\ Measurement unit: i2c_sclk \\\\"]
pub type SclMainStToW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Configures the threshold value of SCL_MAIN_FSM state unchanged period. It should be no more than 23.\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn scl_main_st_to(&self) -> SclMainStToR {
        SclMainStToR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the threshold value of SCL_MAIN_FSM state unchanged period. It should be no more than 23.\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn scl_main_st_to(&mut self) -> SclMainStToW<'_, SclMainStTimeOutSpec> {
        SclMainStToW::new(self, 0)
    }
}
#[doc = "SCL main status time out register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_main_st_time_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_main_st_time_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclMainStTimeOutSpec;
impl crate::RegisterSpec for SclMainStTimeOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_main_st_time_out::R`](R) reader structure"]
impl crate::Readable for SclMainStTimeOutSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_main_st_time_out::W`](W) writer structure"]
impl crate::Writable for SclMainStTimeOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_MAIN_ST_TIME_OUT to value 0x10"]
impl crate::Resettable for SclMainStTimeOutSpec {
    const RESET_VALUE: u32 = 0x10;
}

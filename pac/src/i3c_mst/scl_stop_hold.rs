#[doc = "Register `SCL_STOP_HOLD` reader"]
pub type R = crate::R<SclStopHoldSpec>;
#[doc = "Register `SCL_STOP_HOLD` writer"]
pub type W = crate::W<SclStopHoldSpec>;
#[doc = "Field `REG_SCL_STOP_HOLD_TIME` reader - I2C_SCL_STOP_HOLD_TIME"]
pub type RegSclStopHoldTimeR = crate::FieldReader<u16>;
#[doc = "Field `REG_SCL_STOP_HOLD_TIME` writer - I2C_SCL_STOP_HOLD_TIME"]
pub type RegSclStopHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - I2C_SCL_STOP_HOLD_TIME"]
    #[inline(always)]
    pub fn reg_scl_stop_hold_time(&self) -> RegSclStopHoldTimeR {
        RegSclStopHoldTimeR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C_SCL_STOP_HOLD_TIME"]
    #[inline(always)]
    pub fn reg_scl_stop_hold_time(&mut self) -> RegSclStopHoldTimeW<'_, SclStopHoldSpec> {
        RegSclStopHoldTimeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_stop_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_stop_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclStopHoldSpec;
impl crate::RegisterSpec for SclStopHoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stop_hold::R`](R) reader structure"]
impl crate::Readable for SclStopHoldSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_stop_hold::W`](W) writer structure"]
impl crate::Writable for SclStopHoldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_STOP_HOLD to value 0x08"]
impl crate::Resettable for SclStopHoldSpec {
    const RESET_VALUE: u32 = 0x08;
}

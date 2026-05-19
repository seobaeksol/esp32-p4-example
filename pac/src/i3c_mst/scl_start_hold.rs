#[doc = "Register `SCL_START_HOLD` reader"]
pub type R = crate::R<SclStartHoldSpec>;
#[doc = "Register `SCL_START_HOLD` writer"]
pub type W = crate::W<SclStartHoldSpec>;
#[doc = "Field `REG_SCL_START_HOLD_TIME` reader - I2C_SCL_START_HOLD_TIME"]
pub type RegSclStartHoldTimeR = crate::FieldReader<u16>;
#[doc = "Field `REG_SCL_START_HOLD_TIME` writer - I2C_SCL_START_HOLD_TIME"]
pub type RegSclStartHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `REG_START_DET_HOLD_TIME` reader - NA"]
pub type RegStartDetHoldTimeR = crate::FieldReader;
#[doc = "Field `REG_START_DET_HOLD_TIME` writer - NA"]
pub type RegStartDetHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - I2C_SCL_START_HOLD_TIME"]
    #[inline(always)]
    pub fn reg_scl_start_hold_time(&self) -> RegSclStartHoldTimeR {
        RegSclStartHoldTimeR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:10 - NA"]
    #[inline(always)]
    pub fn reg_start_det_hold_time(&self) -> RegStartDetHoldTimeR {
        RegStartDetHoldTimeR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C_SCL_START_HOLD_TIME"]
    #[inline(always)]
    pub fn reg_scl_start_hold_time(&mut self) -> RegSclStartHoldTimeW<'_, SclStartHoldSpec> {
        RegSclStartHoldTimeW::new(self, 0)
    }
    #[doc = "Bits 9:10 - NA"]
    #[inline(always)]
    pub fn reg_start_det_hold_time(&mut self) -> RegStartDetHoldTimeW<'_, SclStartHoldSpec> {
        RegStartDetHoldTimeW::new(self, 9)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_start_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_start_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclStartHoldSpec;
impl crate::RegisterSpec for SclStartHoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_start_hold::R`](R) reader structure"]
impl crate::Readable for SclStartHoldSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_start_hold::W`](W) writer structure"]
impl crate::Writable for SclStartHoldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_START_HOLD to value 0x08"]
impl crate::Resettable for SclStartHoldSpec {
    const RESET_VALUE: u32 = 0x08;
}

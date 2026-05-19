#[doc = "Register `HW_STANDBY_CNT` reader"]
pub type R = crate::R<HwStandbyCntSpec>;
#[doc = "Register `HW_STANDBY_CNT` writer"]
pub type W = crate::W<HwStandbyCntSpec>;
#[doc = "Field `STANDBY_WAIT_CNT` reader - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
pub type StandbyWaitCntR = crate::FieldReader<u32>;
#[doc = "Field `STANDBY_WAIT_CNT` writer - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
pub type StandbyWaitCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
    #[inline(always)]
    pub fn standby_wait_cnt(&self) -> StandbyWaitCntR {
        StandbyWaitCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled."]
    #[inline(always)]
    pub fn standby_wait_cnt(&mut self) -> StandbyWaitCntW<'_, HwStandbyCntSpec> {
        StandbyWaitCntW::new(self, 0)
    }
}
#[doc = "Configure standby counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_standby_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_standby_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwStandbyCntSpec;
impl crate::RegisterSpec for HwStandbyCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_standby_cnt::R`](R) reader structure"]
impl crate::Readable for HwStandbyCntSpec {}
#[doc = "`write(|w| ..)` method takes [`hw_standby_cnt::W`](W) writer structure"]
impl crate::Writable for HwStandbyCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_STANDBY_CNT to value 0x01"]
impl crate::Resettable for HwStandbyCntSpec {
    const RESET_VALUE: u32 = 0x01;
}

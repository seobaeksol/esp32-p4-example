#[doc = "Register `LOWPULSE` reader"]
pub type R = crate::R<LowpulseSpec>;
#[doc = "Field `MIN_CNT` reader - This register stores the value of the minimum duration time of the low level pulse. It is used in baud rate-detect process."]
pub type MinCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - This register stores the value of the minimum duration time of the low level pulse. It is used in baud rate-detect process."]
    #[inline(always)]
    pub fn min_cnt(&self) -> MinCntR {
        MinCntR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Autobaud minimum low pulse duration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpulse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowpulseSpec;
impl crate::RegisterSpec for LowpulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowpulse::R`](R) reader structure"]
impl crate::Readable for LowpulseSpec {}
#[doc = "`reset()` method sets LOWPULSE to value 0x0fff"]
impl crate::Resettable for LowpulseSpec {
    const RESET_VALUE: u32 = 0x0fff;
}

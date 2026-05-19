#[doc = "Register `TIMESTAMP_CFG` reader"]
pub type R = crate::R<TimestampCfgSpec>;
#[doc = "Register `TIMESTAMP_CFG` writer"]
pub type W = crate::W<TimestampCfgSpec>;
#[doc = "Field `TS_ENABLE` reader - enable the timestamp collection function."]
pub type TsEnableR = crate::BitReader;
#[doc = "Field `TS_ENABLE` writer - enable the timestamp collection function."]
pub type TsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable the timestamp collection function."]
    #[inline(always)]
    pub fn ts_enable(&self) -> TsEnableR {
        TsEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable the timestamp collection function."]
    #[inline(always)]
    pub fn ts_enable(&mut self) -> TsEnableW<'_, TimestampCfgSpec> {
        TsEnableW::new(self, 0)
    }
}
#[doc = "Timestamp configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampCfgSpec;
impl crate::RegisterSpec for TimestampCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_cfg::R`](R) reader structure"]
impl crate::Readable for TimestampCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`timestamp_cfg::W`](W) writer structure"]
impl crate::Writable for TimestampCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMESTAMP_CFG to value 0"]
impl crate::Resettable for TimestampCfgSpec {}

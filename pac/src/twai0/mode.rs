#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `RESET_MODE` reader - 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode."]
pub type ResetModeR = crate::BitReader;
#[doc = "Field `RESET_MODE` writer - 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode."]
pub type ResetModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LISTEN_ONLY_MODE` reader - 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal."]
pub type ListenOnlyModeR = crate::BitReader;
#[doc = "Field `LISTEN_ONLY_MODE` writer - 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal."]
pub type ListenOnlyModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELF_TEST_MODE` reader - 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission."]
pub type SelfTestModeR = crate::BitReader;
#[doc = "Field `SELF_TEST_MODE` writer - 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission."]
pub type SelfTestModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCEPTANCE_FILTER_MODE` reader - 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active)."]
pub type AcceptanceFilterModeR = crate::BitReader;
#[doc = "Field `ACCEPTANCE_FILTER_MODE` writer - 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active)."]
pub type AcceptanceFilterModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode."]
    #[inline(always)]
    pub fn reset_mode(&self) -> ResetModeR {
        ResetModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal."]
    #[inline(always)]
    pub fn listen_only_mode(&self) -> ListenOnlyModeR {
        ListenOnlyModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission."]
    #[inline(always)]
    pub fn self_test_mode(&self) -> SelfTestModeR {
        SelfTestModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active)."]
    #[inline(always)]
    pub fn acceptance_filter_mode(&self) -> AcceptanceFilterModeR {
        AcceptanceFilterModeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: reset, detection of a set reset mode bit results in aborting the current transmission/reception of a message and entering the reset mode. 0: normal, on the '1-to-0' transition of the reset mode bit, the TWAI controller returns to the operating mode."]
    #[inline(always)]
    pub fn reset_mode(&mut self) -> ResetModeW<'_, ModeSpec> {
        ResetModeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: listen only, in this mode the TWAI controller would give no acknowledge to the TWAI-bus, even if a message is received successfully. The error counters are stopped at the current value. 0: normal."]
    #[inline(always)]
    pub fn listen_only_mode(&mut self) -> ListenOnlyModeW<'_, ModeSpec> {
        ListenOnlyModeW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: self test, in this mode a full node test is possible without any other active node on the bus using the self reception request command. The TWAI controller will perform a successful transmission, even if there is no acknowledge received. 0: normal, an acknowledge is required for successful transmission."]
    #[inline(always)]
    pub fn self_test_mode(&mut self) -> SelfTestModeW<'_, ModeSpec> {
        SelfTestModeW::new(self, 2)
    }
    #[doc = "Bit 3 - 1:single, the single acceptance filter option is enabled (one filter with the length of 32 bit is active). 0:dual, the dual acceptance filter option is enabled (two filters, each with the length of 16 bit are active)."]
    #[inline(always)]
    pub fn acceptance_filter_mode(&mut self) -> AcceptanceFilterModeW<'_, ModeSpec> {
        AcceptanceFilterModeW::new(self, 3)
    }
}
#[doc = "TWAI mode register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x01;
}

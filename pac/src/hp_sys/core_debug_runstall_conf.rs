#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` reader"]
pub type R = crate::R<CoreDebugRunstallConfSpec>;
#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` writer"]
pub type W = crate::W<CoreDebugRunstallConfSpec>;
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` reader - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
pub type CoreDebugRunstallEnableR = crate::BitReader;
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` writer - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
pub type CoreDebugRunstallEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
    #[inline(always)]
    pub fn core_debug_runstall_enable(&self) -> CoreDebugRunstallEnableR {
        CoreDebugRunstallEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
    #[inline(always)]
    pub fn core_debug_runstall_enable(
        &mut self,
    ) -> CoreDebugRunstallEnableW<'_, CoreDebugRunstallConfSpec> {
        CoreDebugRunstallEnableW::new(self, 0)
    }
}
#[doc = "Core Debug runstall configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_debug_runstall_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreDebugRunstallConfSpec;
impl crate::RegisterSpec for CoreDebugRunstallConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_debug_runstall_conf::R`](R) reader structure"]
impl crate::Readable for CoreDebugRunstallConfSpec {}
#[doc = "`write(|w| ..)` method takes [`core_debug_runstall_conf::W`](W) writer structure"]
impl crate::Writable for CoreDebugRunstallConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_DEBUG_RUNSTALL_CONF to value 0"]
impl crate::Resettable for CoreDebugRunstallConfSpec {}

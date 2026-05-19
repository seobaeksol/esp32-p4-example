#[doc = "Register `FILTER_CONTROL` reader"]
pub type R = crate::R<FilterControlSpec>;
#[doc = "Register `FILTER_CONTROL` writer"]
pub type W = crate::W<FilterControlSpec>;
#[doc = "Field `FILTER_EN` reader - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
pub type FilterEnR = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
pub type FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_COMP` reader - when set, the comparator must be high in order for the filter to match"]
pub type MatchCompR = crate::BitReader;
#[doc = "Field `MATCH_COMP` writer - when set, the comparator must be high in order for the filter to match"]
pub type MatchCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_PRIVILEGE` reader - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
pub type MatchPrivilegeR = crate::BitReader;
#[doc = "Field `MATCH_PRIVILEGE` writer - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
pub type MatchPrivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_ECAUSE` reader - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
pub type MatchEcauseR = crate::BitReader;
#[doc = "Field `MATCH_ECAUSE` writer - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
pub type MatchEcauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_INTERRUPT` reader - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
pub type MatchInterruptR = crate::BitReader;
#[doc = "Field `MATCH_INTERRUPT` writer - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
pub type MatchInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
    #[inline(always)]
    pub fn filter_en(&self) -> FilterEnR {
        FilterEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when set, the comparator must be high in order for the filter to match"]
    #[inline(always)]
    pub fn match_comp(&self) -> MatchCompR {
        MatchCompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
    #[inline(always)]
    pub fn match_privilege(&self) -> MatchPrivilegeR {
        MatchPrivilegeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
    #[inline(always)]
    pub fn match_ecause(&self) -> MatchEcauseR {
        MatchEcauseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
    #[inline(always)]
    pub fn match_interrupt(&self) -> MatchInterruptR {
        MatchInterruptR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FilterEnW<'_, FilterControlSpec> {
        FilterEnW::new(self, 0)
    }
    #[doc = "Bit 1 - when set, the comparator must be high in order for the filter to match"]
    #[inline(always)]
    pub fn match_comp(&mut self) -> MatchCompW<'_, FilterControlSpec> {
        MatchCompW::new(self, 1)
    }
    #[doc = "Bit 2 - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
    #[inline(always)]
    pub fn match_privilege(&mut self) -> MatchPrivilegeW<'_, FilterControlSpec> {
        MatchPrivilegeW::new(self, 2)
    }
    #[doc = "Bit 3 - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
    #[inline(always)]
    pub fn match_ecause(&mut self) -> MatchEcauseW<'_, FilterControlSpec> {
        MatchEcauseW::new(self, 3)
    }
    #[doc = "Bit 4 - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
    #[inline(always)]
    pub fn match_interrupt(&mut self) -> MatchInterruptW<'_, FilterControlSpec> {
        MatchInterruptW::new(self, 4)
    }
}
#[doc = "filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterControlSpec;
impl crate::RegisterSpec for FilterControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_control::R`](R) reader structure"]
impl crate::Readable for FilterControlSpec {}
#[doc = "`write(|w| ..)` method takes [`filter_control::W`](W) writer structure"]
impl crate::Writable for FilterControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_CONTROL to value 0"]
impl crate::Resettable for FilterControlSpec {}

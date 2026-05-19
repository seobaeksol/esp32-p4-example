#[doc = "Register `FILTER_COMPARATOR_CONTROL` reader"]
pub type R = crate::R<FilterComparatorControlSpec>;
#[doc = "Register `FILTER_COMPARATOR_CONTROL` writer"]
pub type W = crate::W<FilterComparatorControlSpec>;
#[doc = "Field `P_INPUT` reader - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type PInputR = crate::BitReader;
#[doc = "Field `P_INPUT` writer - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type PInputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_FUNCTION` reader - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type PFunctionR = crate::FieldReader;
#[doc = "Field `P_FUNCTION` writer - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type PFunctionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `P_NOTIFY` reader - Generate a trace packet explicitly reporting the address that cause the primary match"]
pub type PNotifyR = crate::BitReader;
#[doc = "Field `P_NOTIFY` writer - Generate a trace packet explicitly reporting the address that cause the primary match"]
pub type PNotifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_INPUT` reader - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type SInputR = crate::BitReader;
#[doc = "Field `S_INPUT` writer - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type SInputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_FUNCTION` reader - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type SFunctionR = crate::FieldReader;
#[doc = "Field `S_FUNCTION` writer - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type SFunctionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `S_NOTIFY` reader - Generate a trace packet explicitly reporting the address that cause the secondary match"]
pub type SNotifyR = crate::BitReader;
#[doc = "Field `S_NOTIFY` writer - Generate a trace packet explicitly reporting the address that cause the secondary match"]
pub type SNotifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_MODE` reader - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&\\&S),\\\\ 2:either primary or secondary comparator matches !(P\\&\\&S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
pub type MatchModeR = crate::FieldReader;
#[doc = "Field `MATCH_MODE` writer - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&\\&S),\\\\ 2:either primary or secondary comparator matches !(P\\&\\&S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
pub type MatchModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    pub fn p_input(&self) -> PInputR {
        PInputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    pub fn p_function(&self) -> PFunctionR {
        PFunctionR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Generate a trace packet explicitly reporting the address that cause the primary match"]
    #[inline(always)]
    pub fn p_notify(&self) -> PNotifyR {
        PNotifyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    pub fn s_input(&self) -> SInputR {
        SInputR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    pub fn s_function(&self) -> SFunctionR {
        SFunctionR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - Generate a trace packet explicitly reporting the address that cause the secondary match"]
    #[inline(always)]
    pub fn s_notify(&self) -> SNotifyR {
        SNotifyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&\\&S),\\\\ 2:either primary or secondary comparator matches !(P\\&\\&S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
    #[inline(always)]
    pub fn match_mode(&self) -> MatchModeR {
        MatchModeR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    pub fn p_input(&mut self) -> PInputW<'_, FilterComparatorControlSpec> {
        PInputW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    pub fn p_function(&mut self) -> PFunctionW<'_, FilterComparatorControlSpec> {
        PFunctionW::new(self, 2)
    }
    #[doc = "Bit 5 - Generate a trace packet explicitly reporting the address that cause the primary match"]
    #[inline(always)]
    pub fn p_notify(&mut self) -> PNotifyW<'_, FilterComparatorControlSpec> {
        PNotifyW::new(self, 5)
    }
    #[doc = "Bit 8 - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    pub fn s_input(&mut self) -> SInputW<'_, FilterComparatorControlSpec> {
        SInputW::new(self, 8)
    }
    #[doc = "Bits 10:12 - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    pub fn s_function(&mut self) -> SFunctionW<'_, FilterComparatorControlSpec> {
        SFunctionW::new(self, 10)
    }
    #[doc = "Bit 13 - Generate a trace packet explicitly reporting the address that cause the secondary match"]
    #[inline(always)]
    pub fn s_notify(&mut self) -> SNotifyW<'_, FilterComparatorControlSpec> {
        SNotifyW::new(self, 13)
    }
    #[doc = "Bits 16:17 - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&\\&S),\\\\ 2:either primary or secondary comparator matches !(P\\&\\&S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
    #[inline(always)]
    pub fn match_mode(&mut self) -> MatchModeW<'_, FilterComparatorControlSpec> {
        MatchModeW::new(self, 16)
    }
}
#[doc = "filter comparator match control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_comparator_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_comparator_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterComparatorControlSpec;
impl crate::RegisterSpec for FilterComparatorControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_comparator_control::R`](R) reader structure"]
impl crate::Readable for FilterComparatorControlSpec {}
#[doc = "`write(|w| ..)` method takes [`filter_comparator_control::W`](W) writer structure"]
impl crate::Writable for FilterComparatorControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_COMPARATOR_CONTROL to value 0"]
impl crate::Resettable for FilterComparatorControlSpec {}

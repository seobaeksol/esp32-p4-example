#[doc = "Register `HP2LP_WAKEUP_GROUP3_EN` reader"]
pub type R = crate::R<Hp2lpWakeupGroup3EnSpec>;
#[doc = "Register `HP2LP_WAKEUP_GROUP3_EN` writer"]
pub type W = crate::W<Hp2lpWakeupGroup3EnSpec>;
#[doc = "Field `H2LP_WAKEUP_GROUP3_EN` reader - Set each bit to enable corresponding peripheral wakeup to PMU."]
pub type H2lpWakeupGroup3EnR = crate::FieldReader<u16>;
#[doc = "Field `H2LP_WAKEUP_GROUP3_EN` writer - Set each bit to enable corresponding peripheral wakeup to PMU."]
pub type H2lpWakeupGroup3EnW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Set each bit to enable corresponding peripheral wakeup to PMU."]
    #[inline(always)]
    pub fn h2lp_wakeup_group3_en(&self) -> H2lpWakeupGroup3EnR {
        H2lpWakeupGroup3EnR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Set each bit to enable corresponding peripheral wakeup to PMU."]
    #[inline(always)]
    pub fn h2lp_wakeup_group3_en(&mut self) -> H2lpWakeupGroup3EnW<'_, Hp2lpWakeupGroup3EnSpec> {
        H2lpWakeupGroup3EnW::new(self, 0)
    }
}
#[doc = "HpP2LP Wakeup Enable Register Group3\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_wakeup_group3_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_wakeup_group3_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hp2lpWakeupGroup3EnSpec;
impl crate::RegisterSpec for Hp2lpWakeupGroup3EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2lp_wakeup_group3_en::R`](R) reader structure"]
impl crate::Readable for Hp2lpWakeupGroup3EnSpec {}
#[doc = "`write(|w| ..)` method takes [`hp2lp_wakeup_group3_en::W`](W) writer structure"]
impl crate::Writable for Hp2lpWakeupGroup3EnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP2LP_WAKEUP_GROUP3_EN to value 0"]
impl crate::Resettable for Hp2lpWakeupGroup3EnSpec {}

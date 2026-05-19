#[doc = "Register `ARB` reader"]
pub type R = crate::R<ArbSpec>;
#[doc = "Register `ARB` writer"]
pub type W = crate::W<ArbSpec>;
#[doc = "Field `IN_ARB_TOKEN_NUM` reader - Set the max number of token count of arbiter"]
pub type InArbTokenNumR = crate::FieldReader;
#[doc = "Field `IN_ARB_TOKEN_NUM` writer - Set the max number of token count of arbiter"]
pub type InArbTokenNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTER_IN_ARB_PRIORITY` reader - Set the priority of channel"]
pub type InterInArbPriorityR = crate::FieldReader;
#[doc = "Field `INTER_IN_ARB_PRIORITY` writer - Set the priority of channel"]
pub type InterInArbPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn in_arb_token_num(&self) -> InArbTokenNumR {
        InArbTokenNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - Set the priority of channel"]
    #[inline(always)]
    pub fn inter_in_arb_priority(&self) -> InterInArbPriorityR {
        InterInArbPriorityR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn in_arb_token_num(&mut self) -> InArbTokenNumW<'_, ArbSpec> {
        InArbTokenNumW::new(self, 0)
    }
    #[doc = "Bits 6:8 - Set the priority of channel"]
    #[inline(always)]
    pub fn inter_in_arb_priority(&mut self) -> InterInArbPriorityW<'_, ArbSpec> {
        InterInArbPriorityW::new(self, 6)
    }
}
#[doc = "RX CH5 arb register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbSpec;
impl crate::RegisterSpec for ArbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb::R`](R) reader structure"]
impl crate::Readable for ArbSpec {}
#[doc = "`write(|w| ..)` method takes [`arb::W`](W) writer structure"]
impl crate::Writable for ArbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARB to value 0x41"]
impl crate::Resettable for ArbSpec {
    const RESET_VALUE: u32 = 0x41;
}

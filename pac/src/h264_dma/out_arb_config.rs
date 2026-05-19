#[doc = "Register `OUT_ARB_CONFIG` reader"]
pub type R = crate::R<OutArbConfigSpec>;
#[doc = "Register `OUT_ARB_CONFIG` writer"]
pub type W = crate::W<OutArbConfigSpec>;
#[doc = "Field `OUT_ARB_TIMEOUT_NUM` reader - Set the max number of timeout count of arbiter"]
pub type OutArbTimeoutNumR = crate::FieldReader<u16>;
#[doc = "Field `OUT_ARB_TIMEOUT_NUM` writer - Set the max number of timeout count of arbiter"]
pub type OutArbTimeoutNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUT_WEIGHT_EN` reader - reserved"]
pub type OutWeightEnR = crate::BitReader;
#[doc = "Field `OUT_WEIGHT_EN` writer - reserved"]
pub type OutWeightEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Set the max number of timeout count of arbiter"]
    #[inline(always)]
    pub fn out_arb_timeout_num(&self) -> OutArbTimeoutNumR {
        OutArbTimeoutNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    pub fn out_weight_en(&self) -> OutWeightEnR {
        OutWeightEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the max number of timeout count of arbiter"]
    #[inline(always)]
    pub fn out_arb_timeout_num(&mut self) -> OutArbTimeoutNumW<'_, OutArbConfigSpec> {
        OutArbTimeoutNumW::new(self, 0)
    }
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    pub fn out_weight_en(&mut self) -> OutWeightEnW<'_, OutArbConfigSpec> {
        OutWeightEnW::new(self, 16)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`out_arb_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_arb_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutArbConfigSpec;
impl crate::RegisterSpec for OutArbConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_arb_config::R`](R) reader structure"]
impl crate::Readable for OutArbConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`out_arb_config::W`](W) writer structure"]
impl crate::Writable for OutArbConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_ARB_CONFIG to value 0"]
impl crate::Resettable for OutArbConfigSpec {}

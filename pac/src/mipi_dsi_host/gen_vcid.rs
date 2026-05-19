#[doc = "Register `GEN_VCID` reader"]
pub type R = crate::R<GenVcidSpec>;
#[doc = "Register `GEN_VCID` writer"]
pub type W = crate::W<GenVcidSpec>;
#[doc = "Field `RX` reader - NA"]
pub type RxR = crate::FieldReader;
#[doc = "Field `RX` writer - NA"]
pub type RxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TEAR_AUTO` reader - NA"]
pub type TearAutoR = crate::FieldReader;
#[doc = "Field `TEAR_AUTO` writer - NA"]
pub type TearAutoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_AUTO` reader - NA"]
pub type TxAutoR = crate::FieldReader;
#[doc = "Field `TX_AUTO` writer - NA"]
pub type TxAutoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - NA"]
    #[inline(always)]
    pub fn tear_auto(&self) -> TearAutoR {
        TearAutoR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    pub fn tx_auto(&self) -> TxAutoR {
        TxAutoR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, GenVcidSpec> {
        RxW::new(self, 0)
    }
    #[doc = "Bits 8:9 - NA"]
    #[inline(always)]
    pub fn tear_auto(&mut self) -> TearAutoW<'_, GenVcidSpec> {
        TearAutoW::new(self, 8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    pub fn tx_auto(&mut self) -> TxAutoW<'_, GenVcidSpec> {
        TxAutoW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_vcid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_vcid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenVcidSpec;
impl crate::RegisterSpec for GenVcidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_vcid::R`](R) reader structure"]
impl crate::Readable for GenVcidSpec {}
#[doc = "`write(|w| ..)` method takes [`gen_vcid::W`](W) writer structure"]
impl crate::Writable for GenVcidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN_VCID to value 0"]
impl crate::Resettable for GenVcidSpec {}

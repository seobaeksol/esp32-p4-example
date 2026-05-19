#[doc = "Register `RDN_RESULT` reader"]
pub type R = crate::R<RdnResultSpec>;
#[doc = "Register `RDN_RESULT` writer"]
pub type W = crate::W<RdnResultSpec>;
#[doc = "Field `RDN_ENA` reader - reserved"]
pub type RdnEnaR = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - reserved"]
pub type RdnEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDN_RESULT` reader - reserved"]
pub type RdnResultR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RdnEnaR {
        RdnEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn rdn_result(&self) -> RdnResultR {
        RdnResultR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RdnEnaW<'_, RdnResultSpec> {
        RdnEnaW::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdnResultSpec;
impl crate::RegisterSpec for RdnResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdn_result::R`](R) reader structure"]
impl crate::Readable for RdnResultSpec {}
#[doc = "`write(|w| ..)` method takes [`rdn_result::W`](W) writer structure"]
impl crate::Writable for RdnResultSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDN_RESULT to value 0"]
impl crate::Resettable for RdnResultSpec {}

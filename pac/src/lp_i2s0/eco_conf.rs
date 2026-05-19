#[doc = "Register `ECO_CONF` reader"]
pub type R = crate::R<EcoConfSpec>;
#[doc = "Register `ECO_CONF` writer"]
pub type W = crate::W<EcoConfSpec>;
#[doc = "Field `RDN_ENA` reader - enable rdn counter bit"]
pub type RdnEnaR = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - enable rdn counter bit"]
pub type RdnEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDN_RESULT` reader - rdn result"]
pub type RdnResultR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - enable rdn counter bit"]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RdnEnaR {
        RdnEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rdn result"]
    #[inline(always)]
    pub fn rdn_result(&self) -> RdnResultR {
        RdnResultR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable rdn counter bit"]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RdnEnaW<'_, EcoConfSpec> {
        RdnEnaW::new(self, 0)
    }
}
#[doc = "I2S ECO register\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoConfSpec;
impl crate::RegisterSpec for EcoConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_conf::R`](R) reader structure"]
impl crate::Readable for EcoConfSpec {}
#[doc = "`write(|w| ..)` method takes [`eco_conf::W`](W) writer structure"]
impl crate::Writable for EcoConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_CONF to value 0"]
impl crate::Resettable for EcoConfSpec {}

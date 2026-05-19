#[doc = "Register `ANA_CONF2` reader"]
pub type R = crate::R<AnaConf2Spec>;
#[doc = "Register `ANA_CONF2` writer"]
pub type W = crate::W<AnaConf2Spec>;
#[doc = "Field `ANA_CONF2` reader - need des"]
pub type AnaConf2R = crate::FieldReader<u32>;
#[doc = "Field `ANA_CONF2` writer - need des"]
pub type AnaConf2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ANA_STATUS2` reader - need des"]
pub type AnaStatus2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn ana_conf2(&self) -> AnaConf2R {
        AnaConf2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - need des"]
    #[inline(always)]
    pub fn ana_status2(&self) -> AnaStatus2R {
        AnaStatus2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn ana_conf2(&mut self) -> AnaConf2W<'_, AnaConf2Spec> {
        AnaConf2W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaConf2Spec;
impl crate::RegisterSpec for AnaConf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_conf2::R`](R) reader structure"]
impl crate::Readable for AnaConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`ana_conf2::W`](W) writer structure"]
impl crate::Writable for AnaConf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_CONF2 to value 0"]
impl crate::Resettable for AnaConf2Spec {}

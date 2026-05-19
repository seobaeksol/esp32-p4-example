#[doc = "Register `ANA_CONF0` reader"]
pub type R = crate::R<AnaConf0Spec>;
#[doc = "Register `ANA_CONF0` writer"]
pub type W = crate::W<AnaConf0Spec>;
#[doc = "Field `ANA_CONF0` reader - need des"]
pub type AnaConf0R = crate::FieldReader<u32>;
#[doc = "Field `ANA_CONF0` writer - need des"]
pub type AnaConf0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ANA_STATUS0` reader - need des"]
pub type AnaStatus0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn ana_conf0(&self) -> AnaConf0R {
        AnaConf0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - need des"]
    #[inline(always)]
    pub fn ana_status0(&self) -> AnaStatus0R {
        AnaStatus0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn ana_conf0(&mut self) -> AnaConf0W<'_, AnaConf0Spec> {
        AnaConf0W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaConf0Spec;
impl crate::RegisterSpec for AnaConf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_conf0::R`](R) reader structure"]
impl crate::Readable for AnaConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`ana_conf0::W`](W) writer structure"]
impl crate::Writable for AnaConf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_CONF0 to value 0"]
impl crate::Resettable for AnaConf0Spec {}

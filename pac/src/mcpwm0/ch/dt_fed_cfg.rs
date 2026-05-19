#[doc = "Register `DT_FED_CFG` reader"]
pub type R = crate::R<DtFedCfgSpec>;
#[doc = "Register `DT_FED_CFG` writer"]
pub type W = crate::W<DtFedCfgSpec>;
#[doc = "Field `FED` reader - Configures shadow register for FED."]
pub type FedR = crate::FieldReader<u16>;
#[doc = "Field `FED` writer - Configures shadow register for FED."]
pub type FedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures shadow register for FED."]
    #[inline(always)]
    pub fn fed(&self) -> FedR {
        FedR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures shadow register for FED."]
    #[inline(always)]
    pub fn fed(&mut self) -> FedW<'_, DtFedCfgSpec> {
        FedW::new(self, 0)
    }
}
#[doc = "Falling edge delay (FED) shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_fed_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt_fed_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtFedCfgSpec;
impl crate::RegisterSpec for DtFedCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt_fed_cfg::R`](R) reader structure"]
impl crate::Readable for DtFedCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dt_fed_cfg::W`](W) writer structure"]
impl crate::Writable for DtFedCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT_FED_CFG to value 0"]
impl crate::Resettable for DtFedCfgSpec {}

#[doc = "Register `EVT_ID` reader"]
pub type R = crate::R<EvtIdSpec>;
#[doc = "Register `EVT_ID` writer"]
pub type W = crate::W<EvtIdSpec>;
#[doc = "Field `EVT_ID` reader - Configures ch0_evt_id"]
pub type EvtIdR = crate::FieldReader;
#[doc = "Field `EVT_ID` writer - Configures ch0_evt_id"]
pub type EvtIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures ch0_evt_id"]
    #[inline(always)]
    pub fn evt_id(&self) -> EvtIdR {
        EvtIdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures ch0_evt_id"]
    #[inline(always)]
    pub fn evt_id(&mut self) -> EvtIdW<'_, EvtIdSpec> {
        EvtIdW::new(self, 0)
    }
}
#[doc = "Channel0 event id register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtIdSpec;
impl crate::RegisterSpec for EvtIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_id::R`](R) reader structure"]
impl crate::Readable for EvtIdSpec {}
#[doc = "`write(|w| ..)` method takes [`evt_id::W`](W) writer structure"]
impl crate::Writable for EvtIdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ID to value 0"]
impl crate::Resettable for EvtIdSpec {}

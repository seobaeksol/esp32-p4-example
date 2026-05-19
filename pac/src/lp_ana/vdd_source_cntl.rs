#[doc = "Register `VDD_SOURCE_CNTL` reader"]
pub type R = crate::R<VddSourceCntlSpec>;
#[doc = "Register `VDD_SOURCE_CNTL` writer"]
pub type W = crate::W<VddSourceCntlSpec>;
#[doc = "Field `DETMODE_SEL` reader - need_des"]
pub type DetmodeSelR = crate::FieldReader;
#[doc = "Field `DETMODE_SEL` writer - need_des"]
pub type DetmodeSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VGOOD_EVENT_RECORD` reader - need_des"]
pub type VgoodEventRecordR = crate::FieldReader;
#[doc = "Field `VBAT_EVENT_RECORD_CLR` writer - need_des"]
pub type VbatEventRecordClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOD_SOURCE_ENA` reader - need_des"]
pub type BodSourceEnaR = crate::FieldReader;
#[doc = "Field `BOD_SOURCE_ENA` writer - need_des"]
pub type BodSourceEnaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn detmode_sel(&self) -> DetmodeSelR {
        DetmodeSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn vgood_event_record(&self) -> VgoodEventRecordR {
        VgoodEventRecordR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn bod_source_ena(&self) -> BodSourceEnaR {
        BodSourceEnaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn detmode_sel(&mut self) -> DetmodeSelW<'_, VddSourceCntlSpec> {
        DetmodeSelW::new(self, 0)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn vbat_event_record_clr(&mut self) -> VbatEventRecordClrW<'_, VddSourceCntlSpec> {
        VbatEventRecordClrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn bod_source_ena(&mut self) -> BodSourceEnaW<'_, VddSourceCntlSpec> {
        BodSourceEnaW::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_source_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_source_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VddSourceCntlSpec;
impl crate::RegisterSpec for VddSourceCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_source_cntl::R`](R) reader structure"]
impl crate::Readable for VddSourceCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`vdd_source_cntl::W`](W) writer structure"]
impl crate::Writable for VddSourceCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDD_SOURCE_CNTL to value 0x0400_00ff"]
impl crate::Resettable for VddSourceCntlSpec {
    const RESET_VALUE: u32 = 0x0400_00ff;
}

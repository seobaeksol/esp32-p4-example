#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SLV_EVENT` reader - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
pub type SlvEventR = crate::FieldReader;
#[doc = "Field `SLV_EVENT` writer - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
pub type SlvEventW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTDATA` reader - reserved"]
pub type ExtdataR = crate::BitReader;
#[doc = "Field `EXTDATA` writer - reserved"]
pub type ExtdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAPIDX` reader - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
pub type MapidxR = crate::FieldReader;
#[doc = "Field `MAPIDX` writer - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
pub type MapidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IBIDATA` reader - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
pub type IbidataR = crate::FieldReader;
#[doc = "Field `IBIDATA` writer - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
pub type IbidataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PENDINT` reader - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
pub type PendintR = crate::FieldReader;
#[doc = "Field `PENDINT` writer - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
pub type PendintW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACTSTATE` reader - NA"]
pub type ActstateR = crate::FieldReader;
#[doc = "Field `ACTSTATE` writer - NA"]
pub type ActstateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VENDINFO` reader - NA"]
pub type VendinfoR = crate::FieldReader;
#[doc = "Field `VENDINFO` writer - NA"]
pub type VendinfoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
    #[inline(always)]
    pub fn slv_event(&self) -> SlvEventR {
        SlvEventR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn extdata(&self) -> ExtdataR {
        ExtdataR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
    #[inline(always)]
    pub fn mapidx(&self) -> MapidxR {
        MapidxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
    #[inline(always)]
    pub fn ibidata(&self) -> IbidataR {
        IbidataR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
    #[inline(always)]
    pub fn pendint(&self) -> PendintR {
        PendintR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - NA"]
    #[inline(always)]
    pub fn actstate(&self) -> ActstateR {
        ActstateR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn vendinfo(&self) -> VendinfoR {
        VendinfoR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
    #[inline(always)]
    pub fn slv_event(&mut self) -> SlvEventW<'_, CtrlSpec> {
        SlvEventW::new(self, 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn extdata(&mut self) -> ExtdataW<'_, CtrlSpec> {
        ExtdataW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
    #[inline(always)]
    pub fn mapidx(&mut self) -> MapidxW<'_, CtrlSpec> {
        MapidxW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
    #[inline(always)]
    pub fn ibidata(&mut self) -> IbidataW<'_, CtrlSpec> {
        IbidataW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
    #[inline(always)]
    pub fn pendint(&mut self) -> PendintW<'_, CtrlSpec> {
        PendintW::new(self, 16)
    }
    #[doc = "Bits 20:21 - NA"]
    #[inline(always)]
    pub fn actstate(&mut self) -> ActstateW<'_, CtrlSpec> {
        ActstateW::new(self, 20)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn vendinfo(&mut self) -> VendinfoW<'_, CtrlSpec> {
        VendinfoW::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}

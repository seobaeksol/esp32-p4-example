#[doc = "Register `DPC_DEADPIX_CNT` reader"]
pub type R = crate::R<DpcDeadpixCntSpec>;
#[doc = "Field `DPC_DEADPIX_CNT` reader - this field represents the dead pixel count"]
pub type DpcDeadpixCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - this field represents the dead pixel count"]
    #[inline(always)]
    pub fn dpc_deadpix_cnt(&self) -> DpcDeadpixCntR {
        DpcDeadpixCntR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DPC dead-pix number register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_deadpix_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpcDeadpixCntSpec;
impl crate::RegisterSpec for DpcDeadpixCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc_deadpix_cnt::R`](R) reader structure"]
impl crate::Readable for DpcDeadpixCntSpec {}
#[doc = "`reset()` method sets DPC_DEADPIX_CNT to value 0"]
impl crate::Resettable for DpcDeadpixCntSpec {}

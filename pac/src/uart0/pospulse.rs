#[doc = "Register `POSPULSE` reader"]
pub type R = crate::R<PospulseSpec>;
#[doc = "Field `POSEDGE_MIN_CNT` reader - This register stores the minimal input clock count between two positive edges. It is used in boudrate-detect process."]
pub type PosedgeMinCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - This register stores the minimal input clock count between two positive edges. It is used in boudrate-detect process."]
    #[inline(always)]
    pub fn posedge_min_cnt(&self) -> PosedgeMinCntR {
        PosedgeMinCntR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Autobaud high pulse register\n\nYou can [`read`](crate::Reg::read) this register and get [`pospulse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PospulseSpec;
impl crate::RegisterSpec for PospulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pospulse::R`](R) reader structure"]
impl crate::Readable for PospulseSpec {}
#[doc = "`reset()` method sets POSPULSE to value 0x0fff"]
impl crate::Resettable for PospulseSpec {
    const RESET_VALUE: u32 = 0x0fff;
}

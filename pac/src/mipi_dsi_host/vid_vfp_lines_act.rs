#[doc = "Register `VID_VFP_LINES_ACT` reader"]
pub type R = crate::R<VidVfpLinesActSpec>;
#[doc = "Field `VFP_LINES_ACT` reader - NA"]
pub type VfpLinesActR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vfp_lines_act(&self) -> VfpLinesActR {
        VfpLinesActR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_vfp_lines_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidVfpLinesActSpec;
impl crate::RegisterSpec for VidVfpLinesActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vfp_lines_act::R`](R) reader structure"]
impl crate::Readable for VidVfpLinesActSpec {}
#[doc = "`reset()` method sets VID_VFP_LINES_ACT to value 0"]
impl crate::Resettable for VidVfpLinesActSpec {}

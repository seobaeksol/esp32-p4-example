#[doc = "Register `VID_VACTIVE_LINES_ACT` reader"]
pub type R = crate::R<VidVactiveLinesActSpec>;
#[doc = "Field `V_ACTIVE_LINES_ACT` reader - NA"]
pub type VActiveLinesActR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - NA"]
    #[inline(always)]
    pub fn v_active_lines_act(&self) -> VActiveLinesActR {
        VActiveLinesActR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_vactive_lines_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidVactiveLinesActSpec;
impl crate::RegisterSpec for VidVactiveLinesActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vactive_lines_act::R`](R) reader structure"]
impl crate::Readable for VidVactiveLinesActSpec {}
#[doc = "`reset()` method sets VID_VACTIVE_LINES_ACT to value 0"]
impl crate::Resettable for VidVactiveLinesActSpec {}

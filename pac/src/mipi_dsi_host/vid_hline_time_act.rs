#[doc = "Register `VID_HLINE_TIME_ACT` reader"]
pub type R = crate::R<VidHlineTimeActSpec>;
#[doc = "Field `VID_HLINE_TIME_ACT` reader - NA"]
pub type VidHlineTimeActR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn vid_hline_time_act(&self) -> VidHlineTimeActR {
        VidHlineTimeActR::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_hline_time_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidHlineTimeActSpec;
impl crate::RegisterSpec for VidHlineTimeActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hline_time_act::R`](R) reader structure"]
impl crate::Readable for VidHlineTimeActSpec {}
#[doc = "`reset()` method sets VID_HLINE_TIME_ACT to value 0"]
impl crate::Resettable for VidHlineTimeActSpec {}

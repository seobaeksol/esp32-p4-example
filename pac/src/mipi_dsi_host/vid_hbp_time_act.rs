#[doc = "Register `VID_HBP_TIME_ACT` reader"]
pub type R = crate::R<VidHbpTimeActSpec>;
#[doc = "Field `VID_HBP_TIME_ACT` reader - NA"]
pub type VidHbpTimeActR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    pub fn vid_hbp_time_act(&self) -> VidHbpTimeActR {
        VidHbpTimeActR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_hbp_time_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidHbpTimeActSpec;
impl crate::RegisterSpec for VidHbpTimeActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hbp_time_act::R`](R) reader structure"]
impl crate::Readable for VidHbpTimeActSpec {}
#[doc = "`reset()` method sets VID_HBP_TIME_ACT to value 0"]
impl crate::Resettable for VidHbpTimeActSpec {}

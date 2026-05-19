#[doc = "Register `VID_HBP_TIME` reader"]
pub type R = crate::R<VidHbpTimeSpec>;
#[doc = "Register `VID_HBP_TIME` writer"]
pub type W = crate::W<VidHbpTimeSpec>;
#[doc = "Field `VID_HBP_TIME` reader - NA"]
pub type VidHbpTimeR = crate::FieldReader<u16>;
#[doc = "Field `VID_HBP_TIME` writer - NA"]
pub type VidHbpTimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    pub fn vid_hbp_time(&self) -> VidHbpTimeR {
        VidHbpTimeR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    pub fn vid_hbp_time(&mut self) -> VidHbpTimeW<'_, VidHbpTimeSpec> {
        VidHbpTimeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_hbp_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_hbp_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidHbpTimeSpec;
impl crate::RegisterSpec for VidHbpTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hbp_time::R`](R) reader structure"]
impl crate::Readable for VidHbpTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_hbp_time::W`](W) writer structure"]
impl crate::Writable for VidHbpTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_HBP_TIME to value 0"]
impl crate::Resettable for VidHbpTimeSpec {}

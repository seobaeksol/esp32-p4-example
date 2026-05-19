#[doc = "Register `VID_VFP_LINES` reader"]
pub type R = crate::R<VidVfpLinesSpec>;
#[doc = "Register `VID_VFP_LINES` writer"]
pub type W = crate::W<VidVfpLinesSpec>;
#[doc = "Field `VFP_LINES` reader - NA"]
pub type VfpLinesR = crate::FieldReader<u16>;
#[doc = "Field `VFP_LINES` writer - NA"]
pub type VfpLinesW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vfp_lines(&self) -> VfpLinesR {
        VfpLinesR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vfp_lines(&mut self) -> VfpLinesW<'_, VidVfpLinesSpec> {
        VfpLinesW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_vfp_lines::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_vfp_lines::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidVfpLinesSpec;
impl crate::RegisterSpec for VidVfpLinesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vfp_lines::R`](R) reader structure"]
impl crate::Readable for VidVfpLinesSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_vfp_lines::W`](W) writer structure"]
impl crate::Writable for VidVfpLinesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_VFP_LINES to value 0"]
impl crate::Resettable for VidVfpLinesSpec {}

#[doc = "Register `VID_NULL_SIZE` reader"]
pub type R = crate::R<VidNullSizeSpec>;
#[doc = "Register `VID_NULL_SIZE` writer"]
pub type W = crate::W<VidNullSizeSpec>;
#[doc = "Field `VID_NULL_SIZE` reader - NA"]
pub type VidNullSizeR = crate::FieldReader<u16>;
#[doc = "Field `VID_NULL_SIZE` writer - NA"]
pub type VidNullSizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - NA"]
    #[inline(always)]
    pub fn vid_null_size(&self) -> VidNullSizeR {
        VidNullSizeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - NA"]
    #[inline(always)]
    pub fn vid_null_size(&mut self) -> VidNullSizeW<'_, VidNullSizeSpec> {
        VidNullSizeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_null_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_null_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidNullSizeSpec;
impl crate::RegisterSpec for VidNullSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_null_size::R`](R) reader structure"]
impl crate::Readable for VidNullSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_null_size::W`](W) writer structure"]
impl crate::Writable for VidNullSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_NULL_SIZE to value 0"]
impl crate::Resettable for VidNullSizeSpec {}

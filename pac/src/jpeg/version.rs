#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Register `VERSION` writer"]
pub type W = crate::W<VersionSpec>;
#[doc = "Field `JPEG_VER` reader - Reserved"]
pub type JpegVerR = crate::FieldReader<u32>;
#[doc = "Field `JPEG_VER` writer - Reserved"]
pub type JpegVerW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn jpeg_ver(&self) -> JpegVerR {
        JpegVerR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn jpeg_ver(&mut self) -> JpegVerW<'_, VersionSpec> {
        JpegVerW::new(self, 0)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`write(|w| ..)` method takes [`version::W`](W) writer structure"]
impl crate::Writable for VersionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VERSION to value 0x0241_2260"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0x0241_2260;
}

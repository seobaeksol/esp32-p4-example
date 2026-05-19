#[doc = "Register `SHA_MODE` reader"]
pub type R = crate::R<ShaModeSpec>;
#[doc = "Register `SHA_MODE` writer"]
pub type W = crate::W<ShaModeSpec>;
#[doc = "Field `SHA_MODE` reader - The work mode bits of SHA Calculator in ECDSA Accelerator. 0: SHA1. 1: SHA-224. 2: SHA-256. 3: SHA-384 4: SHA-512. 5: SHA-512224. 6: SHA-512256. 7: invalid."]
pub type ShaModeR = crate::FieldReader;
#[doc = "Field `SHA_MODE` writer - The work mode bits of SHA Calculator in ECDSA Accelerator. 0: SHA1. 1: SHA-224. 2: SHA-256. 3: SHA-384 4: SHA-512. 5: SHA-512224. 6: SHA-512256. 7: invalid."]
pub type ShaModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The work mode bits of SHA Calculator in ECDSA Accelerator. 0: SHA1. 1: SHA-224. 2: SHA-256. 3: SHA-384 4: SHA-512. 5: SHA-512224. 6: SHA-512256. 7: invalid."]
    #[inline(always)]
    pub fn sha_mode(&self) -> ShaModeR {
        ShaModeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The work mode bits of SHA Calculator in ECDSA Accelerator. 0: SHA1. 1: SHA-224. 2: SHA-256. 3: SHA-384 4: SHA-512. 5: SHA-512224. 6: SHA-512256. 7: invalid."]
    #[inline(always)]
    pub fn sha_mode(&mut self) -> ShaModeW<'_, ShaModeSpec> {
        ShaModeW::new(self, 0)
    }
}
#[doc = "ECDSA control SHA register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaModeSpec;
impl crate::RegisterSpec for ShaModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_mode::R`](R) reader structure"]
impl crate::Readable for ShaModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sha_mode::W`](W) writer structure"]
impl crate::Writable for ShaModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_MODE to value 0"]
impl crate::Resettable for ShaModeSpec {}

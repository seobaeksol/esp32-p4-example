#[doc = "Register `T_STRING` reader"]
pub type R = crate::R<TStringSpec>;
#[doc = "Register `T_STRING` writer"]
pub type W = crate::W<TStringSpec>;
#[doc = "Field `T_STRING` reader - Sha t_string (used if and only if mode == SHA_512/t)."]
pub type TStringR = crate::FieldReader<u32>;
#[doc = "Field `T_STRING` writer - Sha t_string (used if and only if mode == SHA_512/t)."]
pub type TStringW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sha t_string (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    pub fn t_string(&self) -> TStringR {
        TStringR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sha t_string (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    pub fn t_string(&mut self) -> TStringW<'_, TStringSpec> {
        TStringW::new(self, 0)
    }
}
#[doc = "SHA 512/t configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`t_string::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_string::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TStringSpec;
impl crate::RegisterSpec for TStringSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_string::R`](R) reader structure"]
impl crate::Readable for TStringSpec {}
#[doc = "`write(|w| ..)` method takes [`t_string::W`](W) writer structure"]
impl crate::Writable for TStringSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T_STRING to value 0"]
impl crate::Resettable for TStringSpec {}

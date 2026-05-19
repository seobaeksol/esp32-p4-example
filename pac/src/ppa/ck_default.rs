#[doc = "Register `CK_DEFAULT` reader"]
pub type R = crate::R<CkDefaultSpec>;
#[doc = "Register `CK_DEFAULT` writer"]
pub type W = crate::W<CkDefaultSpec>;
#[doc = "Field `COLORKEY_DEFAULT_B` reader - default B channle value of color key"]
pub type ColorkeyDefaultBR = crate::FieldReader;
#[doc = "Field `COLORKEY_DEFAULT_B` writer - default B channle value of color key"]
pub type ColorkeyDefaultBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_DEFAULT_G` reader - default G channle value of color key"]
pub type ColorkeyDefaultGR = crate::FieldReader;
#[doc = "Field `COLORKEY_DEFAULT_G` writer - default G channle value of color key"]
pub type ColorkeyDefaultGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_DEFAULT_R` reader - default R channle value of color key"]
pub type ColorkeyDefaultRR = crate::FieldReader;
#[doc = "Field `COLORKEY_DEFAULT_R` writer - default R channle value of color key"]
pub type ColorkeyDefaultRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_FG_BG_REVERSE` reader - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
pub type ColorkeyFgBgReverseR = crate::BitReader;
#[doc = "Field `COLORKEY_FG_BG_REVERSE` writer - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
pub type ColorkeyFgBgReverseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - default B channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_b(&self) -> ColorkeyDefaultBR {
        ColorkeyDefaultBR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - default G channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_g(&self) -> ColorkeyDefaultGR {
        ColorkeyDefaultGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - default R channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_r(&self) -> ColorkeyDefaultRR {
        ColorkeyDefaultRR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
    #[inline(always)]
    pub fn colorkey_fg_bg_reverse(&self) -> ColorkeyFgBgReverseR {
        ColorkeyFgBgReverseR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - default B channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_b(&mut self) -> ColorkeyDefaultBW<'_, CkDefaultSpec> {
        ColorkeyDefaultBW::new(self, 0)
    }
    #[doc = "Bits 8:15 - default G channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_g(&mut self) -> ColorkeyDefaultGW<'_, CkDefaultSpec> {
        ColorkeyDefaultGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - default R channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_r(&mut self) -> ColorkeyDefaultRW<'_, CkDefaultSpec> {
        ColorkeyDefaultRW::new(self, 16)
    }
    #[doc = "Bit 24 - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
    #[inline(always)]
    pub fn colorkey_fg_bg_reverse(&mut self) -> ColorkeyFgBgReverseW<'_, CkDefaultSpec> {
        ColorkeyFgBgReverseW::new(self, 24)
    }
}
#[doc = "default value when foreground and background both in color key range\n\nYou can [`read`](crate::Reg::read) this register and get [`ck_default::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_default::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkDefaultSpec;
impl crate::RegisterSpec for CkDefaultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck_default::R`](R) reader structure"]
impl crate::Readable for CkDefaultSpec {}
#[doc = "`write(|w| ..)` method takes [`ck_default::W`](W) writer structure"]
impl crate::Writable for CkDefaultSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CK_DEFAULT to value 0"]
impl crate::Resettable for CkDefaultSpec {}

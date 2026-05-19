#[doc = "Register `CK_FG_LOW` reader"]
pub type R = crate::R<CkFgLowSpec>;
#[doc = "Register `CK_FG_LOW` writer"]
pub type W = crate::W<CkFgLowSpec>;
#[doc = "Field `COLORKEY_FG_B_LOW` reader - color key lower threshold of foreground b channel"]
pub type ColorkeyFgBLowR = crate::FieldReader;
#[doc = "Field `COLORKEY_FG_B_LOW` writer - color key lower threshold of foreground b channel"]
pub type ColorkeyFgBLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_FG_G_LOW` reader - color key lower threshold of foreground g channel"]
pub type ColorkeyFgGLowR = crate::FieldReader;
#[doc = "Field `COLORKEY_FG_G_LOW` writer - color key lower threshold of foreground g channel"]
pub type ColorkeyFgGLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_FG_R_LOW` reader - color key lower threshold of foreground r channel"]
pub type ColorkeyFgRLowR = crate::FieldReader;
#[doc = "Field `COLORKEY_FG_R_LOW` writer - color key lower threshold of foreground r channel"]
pub type ColorkeyFgRLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - color key lower threshold of foreground b channel"]
    #[inline(always)]
    pub fn colorkey_fg_b_low(&self) -> ColorkeyFgBLowR {
        ColorkeyFgBLowR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - color key lower threshold of foreground g channel"]
    #[inline(always)]
    pub fn colorkey_fg_g_low(&self) -> ColorkeyFgGLowR {
        ColorkeyFgGLowR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - color key lower threshold of foreground r channel"]
    #[inline(always)]
    pub fn colorkey_fg_r_low(&self) -> ColorkeyFgRLowR {
        ColorkeyFgRLowR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - color key lower threshold of foreground b channel"]
    #[inline(always)]
    pub fn colorkey_fg_b_low(&mut self) -> ColorkeyFgBLowW<'_, CkFgLowSpec> {
        ColorkeyFgBLowW::new(self, 0)
    }
    #[doc = "Bits 8:15 - color key lower threshold of foreground g channel"]
    #[inline(always)]
    pub fn colorkey_fg_g_low(&mut self) -> ColorkeyFgGLowW<'_, CkFgLowSpec> {
        ColorkeyFgGLowW::new(self, 8)
    }
    #[doc = "Bits 16:23 - color key lower threshold of foreground r channel"]
    #[inline(always)]
    pub fn colorkey_fg_r_low(&mut self) -> ColorkeyFgRLowW<'_, CkFgLowSpec> {
        ColorkeyFgRLowW::new(self, 16)
    }
}
#[doc = "foreground color key lower threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`ck_fg_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_fg_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkFgLowSpec;
impl crate::RegisterSpec for CkFgLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck_fg_low::R`](R) reader structure"]
impl crate::Readable for CkFgLowSpec {}
#[doc = "`write(|w| ..)` method takes [`ck_fg_low::W`](W) writer structure"]
impl crate::Writable for CkFgLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CK_FG_LOW to value 0x00ff_ffff"]
impl crate::Resettable for CkFgLowSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}

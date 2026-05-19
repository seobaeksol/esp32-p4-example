#[doc = "Register `CK_FG_HIGH` reader"]
pub type R = crate::R<CkFgHighSpec>;
#[doc = "Register `CK_FG_HIGH` writer"]
pub type W = crate::W<CkFgHighSpec>;
#[doc = "Field `COLORKEY_FG_B_HIGH` reader - color key higher threshold of foreground b channel"]
pub type ColorkeyFgBHighR = crate::FieldReader;
#[doc = "Field `COLORKEY_FG_B_HIGH` writer - color key higher threshold of foreground b channel"]
pub type ColorkeyFgBHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_FG_G_HIGH` reader - color key higher threshold of foreground g channel"]
pub type ColorkeyFgGHighR = crate::FieldReader;
#[doc = "Field `COLORKEY_FG_G_HIGH` writer - color key higher threshold of foreground g channel"]
pub type ColorkeyFgGHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_FG_R_HIGH` reader - color key higher threshold of foreground r channel"]
pub type ColorkeyFgRHighR = crate::FieldReader;
#[doc = "Field `COLORKEY_FG_R_HIGH` writer - color key higher threshold of foreground r channel"]
pub type ColorkeyFgRHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - color key higher threshold of foreground b channel"]
    #[inline(always)]
    pub fn colorkey_fg_b_high(&self) -> ColorkeyFgBHighR {
        ColorkeyFgBHighR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - color key higher threshold of foreground g channel"]
    #[inline(always)]
    pub fn colorkey_fg_g_high(&self) -> ColorkeyFgGHighR {
        ColorkeyFgGHighR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - color key higher threshold of foreground r channel"]
    #[inline(always)]
    pub fn colorkey_fg_r_high(&self) -> ColorkeyFgRHighR {
        ColorkeyFgRHighR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - color key higher threshold of foreground b channel"]
    #[inline(always)]
    pub fn colorkey_fg_b_high(&mut self) -> ColorkeyFgBHighW<'_, CkFgHighSpec> {
        ColorkeyFgBHighW::new(self, 0)
    }
    #[doc = "Bits 8:15 - color key higher threshold of foreground g channel"]
    #[inline(always)]
    pub fn colorkey_fg_g_high(&mut self) -> ColorkeyFgGHighW<'_, CkFgHighSpec> {
        ColorkeyFgGHighW::new(self, 8)
    }
    #[doc = "Bits 16:23 - color key higher threshold of foreground r channel"]
    #[inline(always)]
    pub fn colorkey_fg_r_high(&mut self) -> ColorkeyFgRHighW<'_, CkFgHighSpec> {
        ColorkeyFgRHighW::new(self, 16)
    }
}
#[doc = "foreground color key higher threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`ck_fg_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_fg_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkFgHighSpec;
impl crate::RegisterSpec for CkFgHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck_fg_high::R`](R) reader structure"]
impl crate::Readable for CkFgHighSpec {}
#[doc = "`write(|w| ..)` method takes [`ck_fg_high::W`](W) writer structure"]
impl crate::Writable for CkFgHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CK_FG_HIGH to value 0"]
impl crate::Resettable for CkFgHighSpec {}

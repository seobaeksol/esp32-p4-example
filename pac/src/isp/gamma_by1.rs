#[doc = "Register `GAMMA_BY1` reader"]
pub type R = crate::R<GammaBy1Spec>;
#[doc = "Register `GAMMA_BY1` writer"]
pub type W = crate::W<GammaBy1Spec>;
#[doc = "Field `GAMMA_B_Y03` reader - this field configures the point 3 of Y-axis of b channel gamma curve"]
pub type GammaBY03R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y03` writer - this field configures the point 3 of Y-axis of b channel gamma curve"]
pub type GammaBY03W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y02` reader - this field configures the point 2 of Y-axis of b channel gamma curve"]
pub type GammaBY02R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y02` writer - this field configures the point 2 of Y-axis of b channel gamma curve"]
pub type GammaBY02W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y01` reader - this field configures the point 1 of Y-axis of b channel gamma curve"]
pub type GammaBY01R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y01` writer - this field configures the point 1 of Y-axis of b channel gamma curve"]
pub type GammaBY01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y00` reader - this field configures the point 0 of Y-axis of b channel gamma curve"]
pub type GammaBY00R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y00` writer - this field configures the point 0 of Y-axis of b channel gamma curve"]
pub type GammaBY00W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 3 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y03(&self) -> GammaBY03R {
        GammaBY03R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 2 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y02(&self) -> GammaBY02R {
        GammaBY02R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 1 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y01(&self) -> GammaBY01R {
        GammaBY01R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 0 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y00(&self) -> GammaBY00R {
        GammaBY00R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 3 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y03(&mut self) -> GammaBY03W<'_, GammaBy1Spec> {
        GammaBY03W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 2 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y02(&mut self) -> GammaBY02W<'_, GammaBy1Spec> {
        GammaBY02W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 1 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y01(&mut self) -> GammaBY01W<'_, GammaBy1Spec> {
        GammaBY01W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 0 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y00(&mut self) -> GammaBY00W<'_, GammaBy1Spec> {
        GammaBY00W::new(self, 24)
    }
}
#[doc = "point of Y-axis of b channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaBy1Spec;
impl crate::RegisterSpec for GammaBy1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_by1::R`](R) reader structure"]
impl crate::Readable for GammaBy1Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_by1::W`](W) writer structure"]
impl crate::Writable for GammaBy1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_BY1 to value 0x1020_3040"]
impl crate::Resettable for GammaBy1Spec {
    const RESET_VALUE: u32 = 0x1020_3040;
}

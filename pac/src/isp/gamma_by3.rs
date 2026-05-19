#[doc = "Register `GAMMA_BY3` reader"]
pub type R = crate::R<GammaBy3Spec>;
#[doc = "Register `GAMMA_BY3` writer"]
pub type W = crate::W<GammaBy3Spec>;
#[doc = "Field `GAMMA_B_Y0B` reader - this field configures the point 11 of Y-axis of b channel gamma curve"]
pub type GammaBY0bR = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0B` writer - this field configures the point 11 of Y-axis of b channel gamma curve"]
pub type GammaBY0bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y0A` reader - this field configures the point 10 of Y-axis of b channel gamma curve"]
pub type GammaBY0aR = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0A` writer - this field configures the point 10 of Y-axis of b channel gamma curve"]
pub type GammaBY0aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y09` reader - this field configures the point 9 of Y-axis of b channel gamma curve"]
pub type GammaBY09R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y09` writer - this field configures the point 9 of Y-axis of b channel gamma curve"]
pub type GammaBY09W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y08` reader - this field configures the point 8 of Y-axis of b channel gamma curve"]
pub type GammaBY08R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y08` writer - this field configures the point 8 of Y-axis of b channel gamma curve"]
pub type GammaBY08W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 11 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0b(&self) -> GammaBY0bR {
        GammaBY0bR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 10 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0a(&self) -> GammaBY0aR {
        GammaBY0aR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 9 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y09(&self) -> GammaBY09R {
        GammaBY09R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 8 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y08(&self) -> GammaBY08R {
        GammaBY08R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 11 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0b(&mut self) -> GammaBY0bW<'_, GammaBy3Spec> {
        GammaBY0bW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 10 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0a(&mut self) -> GammaBY0aW<'_, GammaBy3Spec> {
        GammaBY0aW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 9 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y09(&mut self) -> GammaBY09W<'_, GammaBy3Spec> {
        GammaBY09W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 8 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y08(&mut self) -> GammaBY08W<'_, GammaBy3Spec> {
        GammaBY08W::new(self, 24)
    }
}
#[doc = "point of Y-axis of b channel gamma curve register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaBy3Spec;
impl crate::RegisterSpec for GammaBy3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_by3::R`](R) reader structure"]
impl crate::Readable for GammaBy3Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_by3::W`](W) writer structure"]
impl crate::Writable for GammaBy3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_BY3 to value 0x90a0_b0c0"]
impl crate::Resettable for GammaBy3Spec {
    const RESET_VALUE: u32 = 0x90a0_b0c0;
}

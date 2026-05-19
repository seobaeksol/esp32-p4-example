#[doc = "Register `GAMMA_GX2` reader"]
pub type R = crate::R<GammaGx2Spec>;
#[doc = "Register `GAMMA_GX2` writer"]
pub type W = crate::W<GammaGx2Spec>;
#[doc = "Field `GAMMA_G_X0F` reader - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0fR = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0F` writer - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0fW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0E` reader - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0eR = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0E` writer - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0eW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0D` reader - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0dR = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0D` writer - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0dW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0C` reader - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0cR = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0C` writer - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0B` reader - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0bR = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0B` writer - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0bW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0A` reader - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0aR = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0A` writer - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX0aW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X09` reader - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX09R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X09` writer - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX09W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X08` reader - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX08R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X08` writer - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaGX08W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0f(&self) -> GammaGX0fR {
        GammaGX0fR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0e(&self) -> GammaGX0eR {
        GammaGX0eR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0d(&self) -> GammaGX0dR {
        GammaGX0dR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0c(&self) -> GammaGX0cR {
        GammaGX0cR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0b(&self) -> GammaGX0bR {
        GammaGX0bR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0a(&self) -> GammaGX0aR {
        GammaGX0aR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x09(&self) -> GammaGX09R {
        GammaGX09R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x08(&self) -> GammaGX08R {
        GammaGX08R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0f(&mut self) -> GammaGX0fW<'_, GammaGx2Spec> {
        GammaGX0fW::new(self, 0)
    }
    #[doc = "Bits 3:5 - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0e(&mut self) -> GammaGX0eW<'_, GammaGx2Spec> {
        GammaGX0eW::new(self, 3)
    }
    #[doc = "Bits 6:8 - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0d(&mut self) -> GammaGX0dW<'_, GammaGx2Spec> {
        GammaGX0dW::new(self, 6)
    }
    #[doc = "Bits 9:11 - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0c(&mut self) -> GammaGX0cW<'_, GammaGx2Spec> {
        GammaGX0cW::new(self, 9)
    }
    #[doc = "Bits 12:14 - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0b(&mut self) -> GammaGX0bW<'_, GammaGx2Spec> {
        GammaGX0bW::new(self, 12)
    }
    #[doc = "Bits 15:17 - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0a(&mut self) -> GammaGX0aW<'_, GammaGx2Spec> {
        GammaGX0aW::new(self, 15)
    }
    #[doc = "Bits 18:20 - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x09(&mut self) -> GammaGX09W<'_, GammaGx2Spec> {
        GammaGX09W::new(self, 18)
    }
    #[doc = "Bits 21:23 - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x08(&mut self) -> GammaGX08W<'_, GammaGx2Spec> {
        GammaGX08W::new(self, 21)
    }
}
#[doc = "point of X-axis of g channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gx2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gx2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaGx2Spec;
impl crate::RegisterSpec for GammaGx2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_gx2::R`](R) reader structure"]
impl crate::Readable for GammaGx2Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_gx2::W`](W) writer structure"]
impl crate::Writable for GammaGx2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_GX2 to value 0x0092_4924"]
impl crate::Resettable for GammaGx2Spec {
    const RESET_VALUE: u32 = 0x0092_4924;
}

#[doc = "Register `GAMMA_RY4` reader"]
pub type R = crate::R<GammaRy4Spec>;
#[doc = "Register `GAMMA_RY4` writer"]
pub type W = crate::W<GammaRy4Spec>;
#[doc = "Field `GAMMA_R_Y0F` reader - this field configures the point 15 of Y-axis of r channel gamma curve"]
pub type GammaRY0fR = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y0F` writer - this field configures the point 15 of Y-axis of r channel gamma curve"]
pub type GammaRY0fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_R_Y0E` reader - this field configures the point 14 of Y-axis of r channel gamma curve"]
pub type GammaRY0eR = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y0E` writer - this field configures the point 14 of Y-axis of r channel gamma curve"]
pub type GammaRY0eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_R_Y0D` reader - this field configures the point 13 of Y-axis of r channel gamma curve"]
pub type GammaRY0dR = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y0D` writer - this field configures the point 13 of Y-axis of r channel gamma curve"]
pub type GammaRY0dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_R_Y0C` reader - this field configures the point 12 of Y-axis of r channel gamma curve"]
pub type GammaRY0cR = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y0C` writer - this field configures the point 12 of Y-axis of r channel gamma curve"]
pub type GammaRY0cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 15 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0f(&self) -> GammaRY0fR {
        GammaRY0fR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 14 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0e(&self) -> GammaRY0eR {
        GammaRY0eR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 13 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0d(&self) -> GammaRY0dR {
        GammaRY0dR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 12 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0c(&self) -> GammaRY0cR {
        GammaRY0cR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 15 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0f(&mut self) -> GammaRY0fW<'_, GammaRy4Spec> {
        GammaRY0fW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 14 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0e(&mut self) -> GammaRY0eW<'_, GammaRy4Spec> {
        GammaRY0eW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 13 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0d(&mut self) -> GammaRY0dW<'_, GammaRy4Spec> {
        GammaRY0dW::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 12 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0c(&mut self) -> GammaRY0cW<'_, GammaRy4Spec> {
        GammaRY0cW::new(self, 24)
    }
}
#[doc = "point of Y-axis of r channel gamma curve register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ry4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ry4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaRy4Spec;
impl crate::RegisterSpec for GammaRy4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_ry4::R`](R) reader structure"]
impl crate::Readable for GammaRy4Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_ry4::W`](W) writer structure"]
impl crate::Writable for GammaRy4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_RY4 to value 0xd0e0_f0ff"]
impl crate::Resettable for GammaRy4Spec {
    const RESET_VALUE: u32 = 0xd0e0_f0ff;
}

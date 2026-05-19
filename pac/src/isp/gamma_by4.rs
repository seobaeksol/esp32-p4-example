#[doc = "Register `GAMMA_BY4` reader"]
pub type R = crate::R<GammaBy4Spec>;
#[doc = "Register `GAMMA_BY4` writer"]
pub type W = crate::W<GammaBy4Spec>;
#[doc = "Field `GAMMA_B_Y0F` reader - this field configures the point 15 of Y-axis of b channel gamma curve"]
pub type GammaBY0fR = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0F` writer - this field configures the point 15 of Y-axis of b channel gamma curve"]
pub type GammaBY0fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y0E` reader - this field configures the point 14 of Y-axis of b channel gamma curve"]
pub type GammaBY0eR = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0E` writer - this field configures the point 14 of Y-axis of b channel gamma curve"]
pub type GammaBY0eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y0D` reader - this field configures the point 13 of Y-axis of b channel gamma curve"]
pub type GammaBY0dR = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0D` writer - this field configures the point 13 of Y-axis of b channel gamma curve"]
pub type GammaBY0dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y0C` reader - this field configures the point 12 of Y-axis of b channel gamma curve"]
pub type GammaBY0cR = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0C` writer - this field configures the point 12 of Y-axis of b channel gamma curve"]
pub type GammaBY0cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 15 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0f(&self) -> GammaBY0fR {
        GammaBY0fR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 14 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0e(&self) -> GammaBY0eR {
        GammaBY0eR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 13 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0d(&self) -> GammaBY0dR {
        GammaBY0dR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 12 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0c(&self) -> GammaBY0cR {
        GammaBY0cR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 15 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0f(&mut self) -> GammaBY0fW<'_, GammaBy4Spec> {
        GammaBY0fW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 14 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0e(&mut self) -> GammaBY0eW<'_, GammaBy4Spec> {
        GammaBY0eW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 13 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0d(&mut self) -> GammaBY0dW<'_, GammaBy4Spec> {
        GammaBY0dW::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 12 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0c(&mut self) -> GammaBY0cW<'_, GammaBy4Spec> {
        GammaBY0cW::new(self, 24)
    }
}
#[doc = "point of Y-axis of b channel gamma curve register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaBy4Spec;
impl crate::RegisterSpec for GammaBy4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_by4::R`](R) reader structure"]
impl crate::Readable for GammaBy4Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_by4::W`](W) writer structure"]
impl crate::Writable for GammaBy4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_BY4 to value 0xd0e0_f0ff"]
impl crate::Resettable for GammaBy4Spec {
    const RESET_VALUE: u32 = 0xd0e0_f0ff;
}

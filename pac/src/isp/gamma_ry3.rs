#[doc = "Register `GAMMA_RY3` reader"]
pub type R = crate::R<GammaRy3Spec>;
#[doc = "Register `GAMMA_RY3` writer"]
pub type W = crate::W<GammaRy3Spec>;
#[doc = "Field `GAMMA_R_Y0B` reader - this field configures the point 11 of Y-axis of r channel gamma curve"]
pub type GammaRY0bR = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y0B` writer - this field configures the point 11 of Y-axis of r channel gamma curve"]
pub type GammaRY0bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_R_Y0A` reader - this field configures the point 10 of Y-axis of r channel gamma curve"]
pub type GammaRY0aR = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y0A` writer - this field configures the point 10 of Y-axis of r channel gamma curve"]
pub type GammaRY0aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_R_Y09` reader - this field configures the point 9 of Y-axis of r channel gamma curve"]
pub type GammaRY09R = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y09` writer - this field configures the point 9 of Y-axis of r channel gamma curve"]
pub type GammaRY09W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_R_Y08` reader - this field configures the point 8 of Y-axis of r channel gamma curve"]
pub type GammaRY08R = crate::FieldReader;
#[doc = "Field `GAMMA_R_Y08` writer - this field configures the point 8 of Y-axis of r channel gamma curve"]
pub type GammaRY08W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 11 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0b(&self) -> GammaRY0bR {
        GammaRY0bR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 10 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0a(&self) -> GammaRY0aR {
        GammaRY0aR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 9 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y09(&self) -> GammaRY09R {
        GammaRY09R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 8 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y08(&self) -> GammaRY08R {
        GammaRY08R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 11 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0b(&mut self) -> GammaRY0bW<'_, GammaRy3Spec> {
        GammaRY0bW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 10 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y0a(&mut self) -> GammaRY0aW<'_, GammaRy3Spec> {
        GammaRY0aW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 9 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y09(&mut self) -> GammaRY09W<'_, GammaRy3Spec> {
        GammaRY09W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 8 of Y-axis of r channel gamma curve"]
    #[inline(always)]
    pub fn gamma_r_y08(&mut self) -> GammaRY08W<'_, GammaRy3Spec> {
        GammaRY08W::new(self, 24)
    }
}
#[doc = "point of Y-axis of r channel gamma curve register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ry3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ry3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaRy3Spec;
impl crate::RegisterSpec for GammaRy3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_ry3::R`](R) reader structure"]
impl crate::Readable for GammaRy3Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_ry3::W`](W) writer structure"]
impl crate::Writable for GammaRy3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_RY3 to value 0x90a0_b0c0"]
impl crate::Resettable for GammaRy3Spec {
    const RESET_VALUE: u32 = 0x90a0_b0c0;
}

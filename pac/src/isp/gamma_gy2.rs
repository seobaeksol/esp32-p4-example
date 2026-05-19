#[doc = "Register `GAMMA_GY2` reader"]
pub type R = crate::R<GammaGy2Spec>;
#[doc = "Register `GAMMA_GY2` writer"]
pub type W = crate::W<GammaGy2Spec>;
#[doc = "Field `GAMMA_G_Y07` reader - this field configures the point 7 of Y-axis of g channel gamma curve"]
pub type GammaGY07R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y07` writer - this field configures the point 7 of Y-axis of g channel gamma curve"]
pub type GammaGY07W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y06` reader - this field configures the point 6 of Y-axis of g channel gamma curve"]
pub type GammaGY06R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y06` writer - this field configures the point 6 of Y-axis of g channel gamma curve"]
pub type GammaGY06W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y05` reader - this field configures the point 5 of Y-axis of g channel gamma curve"]
pub type GammaGY05R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y05` writer - this field configures the point 5 of Y-axis of g channel gamma curve"]
pub type GammaGY05W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y04` reader - this field configures the point 4 of Y-axis of g channel gamma curve"]
pub type GammaGY04R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y04` writer - this field configures the point 4 of Y-axis of g channel gamma curve"]
pub type GammaGY04W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 7 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y07(&self) -> GammaGY07R {
        GammaGY07R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 6 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y06(&self) -> GammaGY06R {
        GammaGY06R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 5 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y05(&self) -> GammaGY05R {
        GammaGY05R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 4 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y04(&self) -> GammaGY04R {
        GammaGY04R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 7 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y07(&mut self) -> GammaGY07W<'_, GammaGy2Spec> {
        GammaGY07W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 6 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y06(&mut self) -> GammaGY06W<'_, GammaGy2Spec> {
        GammaGY06W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 5 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y05(&mut self) -> GammaGY05W<'_, GammaGy2Spec> {
        GammaGY05W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 4 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y04(&mut self) -> GammaGY04W<'_, GammaGy2Spec> {
        GammaGY04W::new(self, 24)
    }
}
#[doc = "point of Y-axis of g channel gamma curve register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gy2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gy2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaGy2Spec;
impl crate::RegisterSpec for GammaGy2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_gy2::R`](R) reader structure"]
impl crate::Readable for GammaGy2Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_gy2::W`](W) writer structure"]
impl crate::Writable for GammaGy2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_GY2 to value 0x5060_7080"]
impl crate::Resettable for GammaGy2Spec {
    const RESET_VALUE: u32 = 0x5060_7080;
}

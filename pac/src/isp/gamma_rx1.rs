#[doc = "Register `GAMMA_RX1` reader"]
pub type R = crate::R<GammaRx1Spec>;
#[doc = "Register `GAMMA_RX1` writer"]
pub type W = crate::W<GammaRx1Spec>;
#[doc = "Field `GAMMA_R_X07` reader - this field configures the point 7 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX07R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X07` writer - this field configures the point 7 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX07W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_R_X06` reader - this field configures the point 6 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX06R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X06` writer - this field configures the point 6 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX06W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_R_X05` reader - this field configures the point 5 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX05R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X05` writer - this field configures the point 5 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX05W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_R_X04` reader - this field configures the point 4 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX04R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X04` writer - this field configures the point 4 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX04W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_R_X03` reader - this field configures the point 3 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX03R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X03` writer - this field configures the point 3 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX03W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_R_X02` reader - this field configures the point 2 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX02R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X02` writer - this field configures the point 2 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX02W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_R_X01` reader - this field configures the point 1 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX01R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X01` writer - this field configures the point 1 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_R_X00` reader - this field configures the point 0 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX00R = crate::FieldReader;
#[doc = "Field `GAMMA_R_X00` writer - this field configures the point 0 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
pub type GammaRX00W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - this field configures the point 7 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x07(&self) -> GammaRX07R {
        GammaRX07R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - this field configures the point 6 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x06(&self) -> GammaRX06R {
        GammaRX06R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - this field configures the point 5 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x05(&self) -> GammaRX05R {
        GammaRX05R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - this field configures the point 4 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x04(&self) -> GammaRX04R {
        GammaRX04R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - this field configures the point 3 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x03(&self) -> GammaRX03R {
        GammaRX03R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - this field configures the point 2 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x02(&self) -> GammaRX02R {
        GammaRX02R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - this field configures the point 1 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x01(&self) -> GammaRX01R {
        GammaRX01R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - this field configures the point 0 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x00(&self) -> GammaRX00R {
        GammaRX00R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - this field configures the point 7 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x07(&mut self) -> GammaRX07W<'_, GammaRx1Spec> {
        GammaRX07W::new(self, 0)
    }
    #[doc = "Bits 3:5 - this field configures the point 6 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x06(&mut self) -> GammaRX06W<'_, GammaRx1Spec> {
        GammaRX06W::new(self, 3)
    }
    #[doc = "Bits 6:8 - this field configures the point 5 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x05(&mut self) -> GammaRX05W<'_, GammaRx1Spec> {
        GammaRX05W::new(self, 6)
    }
    #[doc = "Bits 9:11 - this field configures the point 4 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x04(&mut self) -> GammaRX04W<'_, GammaRx1Spec> {
        GammaRX04W::new(self, 9)
    }
    #[doc = "Bits 12:14 - this field configures the point 3 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x03(&mut self) -> GammaRX03W<'_, GammaRx1Spec> {
        GammaRX03W::new(self, 12)
    }
    #[doc = "Bits 15:17 - this field configures the point 2 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x02(&mut self) -> GammaRX02W<'_, GammaRx1Spec> {
        GammaRX02W::new(self, 15)
    }
    #[doc = "Bits 18:20 - this field configures the point 1 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x01(&mut self) -> GammaRX01W<'_, GammaRx1Spec> {
        GammaRX01W::new(self, 18)
    }
    #[doc = "Bits 21:23 - this field configures the point 0 of X-axis of r channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_r_x00(&mut self) -> GammaRX00W<'_, GammaRx1Spec> {
        GammaRX00W::new(self, 21)
    }
}
#[doc = "point of X-axis of r channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_rx1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_rx1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaRx1Spec;
impl crate::RegisterSpec for GammaRx1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_rx1::R`](R) reader structure"]
impl crate::Readable for GammaRx1Spec {}
#[doc = "`write(|w| ..)` method takes [`gamma_rx1::W`](W) writer structure"]
impl crate::Writable for GammaRx1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_RX1 to value 0x0092_4924"]
impl crate::Resettable for GammaRx1Spec {
    const RESET_VALUE: u32 = 0x0092_4924;
}

#[doc = "Register `BF_GAU0` reader"]
pub type R = crate::R<BfGau0Spec>;
#[doc = "Register `BF_GAU0` writer"]
pub type W = crate::W<BfGau0Spec>;
#[doc = "Field `GAU_TEMPLATE21` reader - this field configures index 21 of gausian template"]
pub type GauTemplate21R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE21` writer - this field configures index 21 of gausian template"]
pub type GauTemplate21W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAU_TEMPLATE20` reader - this field configures index 20 of gausian template"]
pub type GauTemplate20R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE20` writer - this field configures index 20 of gausian template"]
pub type GauTemplate20W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAU_TEMPLATE12` reader - this field configures index 12 of gausian template"]
pub type GauTemplate12R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE12` writer - this field configures index 12 of gausian template"]
pub type GauTemplate12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAU_TEMPLATE11` reader - this field configures index 11 of gausian template"]
pub type GauTemplate11R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE11` writer - this field configures index 11 of gausian template"]
pub type GauTemplate11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAU_TEMPLATE10` reader - this field configures index 10 of gausian template"]
pub type GauTemplate10R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE10` writer - this field configures index 10 of gausian template"]
pub type GauTemplate10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAU_TEMPLATE02` reader - this field configures index 02 of gausian template"]
pub type GauTemplate02R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE02` writer - this field configures index 02 of gausian template"]
pub type GauTemplate02W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAU_TEMPLATE01` reader - this field configures index 01 of gausian template"]
pub type GauTemplate01R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE01` writer - this field configures index 01 of gausian template"]
pub type GauTemplate01W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAU_TEMPLATE00` reader - this field configures index 00 of gausian template"]
pub type GauTemplate00R = crate::FieldReader;
#[doc = "Field `GAU_TEMPLATE00` writer - this field configures index 00 of gausian template"]
pub type GauTemplate00W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - this field configures index 21 of gausian template"]
    #[inline(always)]
    pub fn gau_template21(&self) -> GauTemplate21R {
        GauTemplate21R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - this field configures index 20 of gausian template"]
    #[inline(always)]
    pub fn gau_template20(&self) -> GauTemplate20R {
        GauTemplate20R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - this field configures index 12 of gausian template"]
    #[inline(always)]
    pub fn gau_template12(&self) -> GauTemplate12R {
        GauTemplate12R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - this field configures index 11 of gausian template"]
    #[inline(always)]
    pub fn gau_template11(&self) -> GauTemplate11R {
        GauTemplate11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - this field configures index 10 of gausian template"]
    #[inline(always)]
    pub fn gau_template10(&self) -> GauTemplate10R {
        GauTemplate10R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - this field configures index 02 of gausian template"]
    #[inline(always)]
    pub fn gau_template02(&self) -> GauTemplate02R {
        GauTemplate02R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - this field configures index 01 of gausian template"]
    #[inline(always)]
    pub fn gau_template01(&self) -> GauTemplate01R {
        GauTemplate01R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - this field configures index 00 of gausian template"]
    #[inline(always)]
    pub fn gau_template00(&self) -> GauTemplate00R {
        GauTemplate00R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - this field configures index 21 of gausian template"]
    #[inline(always)]
    pub fn gau_template21(&mut self) -> GauTemplate21W<'_, BfGau0Spec> {
        GauTemplate21W::new(self, 0)
    }
    #[doc = "Bits 4:7 - this field configures index 20 of gausian template"]
    #[inline(always)]
    pub fn gau_template20(&mut self) -> GauTemplate20W<'_, BfGau0Spec> {
        GauTemplate20W::new(self, 4)
    }
    #[doc = "Bits 8:11 - this field configures index 12 of gausian template"]
    #[inline(always)]
    pub fn gau_template12(&mut self) -> GauTemplate12W<'_, BfGau0Spec> {
        GauTemplate12W::new(self, 8)
    }
    #[doc = "Bits 12:15 - this field configures index 11 of gausian template"]
    #[inline(always)]
    pub fn gau_template11(&mut self) -> GauTemplate11W<'_, BfGau0Spec> {
        GauTemplate11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - this field configures index 10 of gausian template"]
    #[inline(always)]
    pub fn gau_template10(&mut self) -> GauTemplate10W<'_, BfGau0Spec> {
        GauTemplate10W::new(self, 16)
    }
    #[doc = "Bits 20:23 - this field configures index 02 of gausian template"]
    #[inline(always)]
    pub fn gau_template02(&mut self) -> GauTemplate02W<'_, BfGau0Spec> {
        GauTemplate02W::new(self, 20)
    }
    #[doc = "Bits 24:27 - this field configures index 01 of gausian template"]
    #[inline(always)]
    pub fn gau_template01(&mut self) -> GauTemplate01W<'_, BfGau0Spec> {
        GauTemplate01W::new(self, 24)
    }
    #[doc = "Bits 28:31 - this field configures index 00 of gausian template"]
    #[inline(always)]
    pub fn gau_template00(&mut self) -> GauTemplate00W<'_, BfGau0Spec> {
        GauTemplate00W::new(self, 28)
    }
}
#[doc = "bf gau template register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bf_gau0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bf_gau0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BfGau0Spec;
impl crate::RegisterSpec for BfGau0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bf_gau0::R`](R) reader structure"]
impl crate::Readable for BfGau0Spec {}
#[doc = "`write(|w| ..)` method takes [`bf_gau0::W`](W) writer structure"]
impl crate::Writable for BfGau0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BF_GAU0 to value 0xffff_ffff"]
impl crate::Resettable for BfGau0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

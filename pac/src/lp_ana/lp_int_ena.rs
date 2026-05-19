#[doc = "Register `LP_INT_ENA` reader"]
pub type R = crate::R<LpIntEnaSpec>;
#[doc = "Register `LP_INT_ENA` writer"]
pub type W = crate::W<LpIntEnaSpec>;
#[doc = "Field `BOD_MODE0` reader - need_des"]
pub type BodMode0R = crate::BitReader;
#[doc = "Field `BOD_MODE0` writer - need_des"]
pub type BodMode0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0(&self) -> BodMode0R {
        BodMode0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0(&mut self) -> BodMode0W<'_, LpIntEnaSpec> {
        BodMode0W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpIntEnaSpec;
impl crate::RegisterSpec for LpIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_ena::R`](R) reader structure"]
impl crate::Readable for LpIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_int_ena::W`](W) writer structure"]
impl crate::Writable for LpIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_ENA to value 0"]
impl crate::Resettable for LpIntEnaSpec {}

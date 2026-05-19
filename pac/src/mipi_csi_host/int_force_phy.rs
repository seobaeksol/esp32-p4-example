#[doc = "Register `INT_FORCE_PHY` reader"]
pub type R = crate::R<IntForcePhySpec>;
#[doc = "Register `INT_FORCE_PHY` writer"]
pub type W = crate::W<IntForcePhySpec>;
#[doc = "Field `FORCE_PHY_ERRSOTHS_0` reader - NA"]
pub type ForcePhyErrsoths0R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRSOTHS_0` writer - NA"]
pub type ForcePhyErrsoths0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PHY_ERRSOTHS_1` reader - NA"]
pub type ForcePhyErrsoths1R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRSOTHS_1` writer - NA"]
pub type ForcePhyErrsoths1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PHY_ERRESC_0` reader - NA"]
pub type ForcePhyErresc0R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRESC_0` writer - NA"]
pub type ForcePhyErresc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PHY_ERRESC_1` reader - NA"]
pub type ForcePhyErresc1R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRESC_1` writer - NA"]
pub type ForcePhyErresc1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_phy_errsoths_0(&self) -> ForcePhyErrsoths0R {
        ForcePhyErrsoths0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_phy_errsoths_1(&self) -> ForcePhyErrsoths1R {
        ForcePhyErrsoths1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn force_phy_erresc_0(&self) -> ForcePhyErresc0R {
        ForcePhyErresc0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn force_phy_erresc_1(&self) -> ForcePhyErresc1R {
        ForcePhyErresc1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_phy_errsoths_0(&mut self) -> ForcePhyErrsoths0W<'_, IntForcePhySpec> {
        ForcePhyErrsoths0W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_phy_errsoths_1(&mut self) -> ForcePhyErrsoths1W<'_, IntForcePhySpec> {
        ForcePhyErrsoths1W::new(self, 1)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn force_phy_erresc_0(&mut self) -> ForcePhyErresc0W<'_, IntForcePhySpec> {
        ForcePhyErresc0W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn force_phy_erresc_1(&mut self) -> ForcePhyErresc1W<'_, IntForcePhySpec> {
        ForcePhyErresc1W::new(self, 17)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_phy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_phy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForcePhySpec;
impl crate::RegisterSpec for IntForcePhySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_phy::R`](R) reader structure"]
impl crate::Readable for IntForcePhySpec {}
#[doc = "`write(|w| ..)` method takes [`int_force_phy::W`](W) writer structure"]
impl crate::Writable for IntForcePhySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_FORCE_PHY to value 0"]
impl crate::Resettable for IntForcePhySpec {}

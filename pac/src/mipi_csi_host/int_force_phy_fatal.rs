#[doc = "Register `INT_FORCE_PHY_FATAL` reader"]
pub type R = crate::R<IntForcePhyFatalSpec>;
#[doc = "Register `INT_FORCE_PHY_FATAL` writer"]
pub type W = crate::W<IntForcePhyFatalSpec>;
#[doc = "Field `FORCE_PHY_ERRSOTSYNCHS_0` reader - NA"]
pub type ForcePhyErrsotsynchs0R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRSOTSYNCHS_0` writer - NA"]
pub type ForcePhyErrsotsynchs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PHY_ERRSOTSYNCHS_1` reader - NA"]
pub type ForcePhyErrsotsynchs1R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRSOTSYNCHS_1` writer - NA"]
pub type ForcePhyErrsotsynchs1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_phy_errsotsynchs_0(&self) -> ForcePhyErrsotsynchs0R {
        ForcePhyErrsotsynchs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_phy_errsotsynchs_1(&self) -> ForcePhyErrsotsynchs1R {
        ForcePhyErrsotsynchs1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_phy_errsotsynchs_0(&mut self) -> ForcePhyErrsotsynchs0W<'_, IntForcePhyFatalSpec> {
        ForcePhyErrsotsynchs0W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_phy_errsotsynchs_1(&mut self) -> ForcePhyErrsotsynchs1W<'_, IntForcePhyFatalSpec> {
        ForcePhyErrsotsynchs1W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_phy_fatal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_phy_fatal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForcePhyFatalSpec;
impl crate::RegisterSpec for IntForcePhyFatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_phy_fatal::R`](R) reader structure"]
impl crate::Readable for IntForcePhyFatalSpec {}
#[doc = "`write(|w| ..)` method takes [`int_force_phy_fatal::W`](W) writer structure"]
impl crate::Writable for IntForcePhyFatalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_FORCE_PHY_FATAL to value 0"]
impl crate::Resettable for IntForcePhyFatalSpec {}

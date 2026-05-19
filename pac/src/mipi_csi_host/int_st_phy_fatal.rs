#[doc = "Register `INT_ST_PHY_FATAL` reader"]
pub type R = crate::R<IntStPhyFatalSpec>;
#[doc = "Field `ST_PHY_ERRSOTSYNCHS_0` reader - NA"]
pub type StPhyErrsotsynchs0R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRSOTSYNCHS_1` reader - NA"]
pub type StPhyErrsotsynchs1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_phy_errsotsynchs_0(&self) -> StPhyErrsotsynchs0R {
        StPhyErrsotsynchs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_phy_errsotsynchs_1(&self) -> StPhyErrsotsynchs1R {
        StPhyErrsotsynchs1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_phy_fatal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStPhyFatalSpec;
impl crate::RegisterSpec for IntStPhyFatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_phy_fatal::R`](R) reader structure"]
impl crate::Readable for IntStPhyFatalSpec {}
#[doc = "`reset()` method sets INT_ST_PHY_FATAL to value 0"]
impl crate::Resettable for IntStPhyFatalSpec {}

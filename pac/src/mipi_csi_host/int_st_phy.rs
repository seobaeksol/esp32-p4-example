#[doc = "Register `INT_ST_PHY` reader"]
pub type R = crate::R<IntStPhySpec>;
#[doc = "Field `ST_PHY_ERRSOTHS_0` reader - NA"]
pub type StPhyErrsoths0R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRSOTHS_1` reader - NA"]
pub type StPhyErrsoths1R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRESC_0` reader - NA"]
pub type StPhyErresc0R = crate::BitReader;
#[doc = "Field `ST_PHY_ERRESC_1` reader - NA"]
pub type StPhyErresc1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_phy_errsoths_0(&self) -> StPhyErrsoths0R {
        StPhyErrsoths0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_phy_errsoths_1(&self) -> StPhyErrsoths1R {
        StPhyErrsoths1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn st_phy_erresc_0(&self) -> StPhyErresc0R {
        StPhyErresc0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn st_phy_erresc_1(&self) -> StPhyErresc1R {
        StPhyErresc1R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_phy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStPhySpec;
impl crate::RegisterSpec for IntStPhySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_phy::R`](R) reader structure"]
impl crate::Readable for IntStPhySpec {}
#[doc = "`reset()` method sets INT_ST_PHY to value 0"]
impl crate::Resettable for IntStPhySpec {}

#[doc = "Register `INT_ST_ECC_CORRECTED` reader"]
pub type R = crate::R<IntStEccCorrectedSpec>;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC0` reader - NA"]
pub type StErrEccCorrectedVc0R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC1` reader - NA"]
pub type StErrEccCorrectedVc1R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC2` reader - NA"]
pub type StErrEccCorrectedVc2R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC3` reader - NA"]
pub type StErrEccCorrectedVc3R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC4` reader - NA"]
pub type StErrEccCorrectedVc4R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC5` reader - NA"]
pub type StErrEccCorrectedVc5R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC6` reader - NA"]
pub type StErrEccCorrectedVc6R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC7` reader - NA"]
pub type StErrEccCorrectedVc7R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC8` reader - NA"]
pub type StErrEccCorrectedVc8R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC9` reader - NA"]
pub type StErrEccCorrectedVc9R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC10` reader - NA"]
pub type StErrEccCorrectedVc10R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC11` reader - NA"]
pub type StErrEccCorrectedVc11R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC12` reader - NA"]
pub type StErrEccCorrectedVc12R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC13` reader - NA"]
pub type StErrEccCorrectedVc13R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC14` reader - NA"]
pub type StErrEccCorrectedVc14R = crate::BitReader;
#[doc = "Field `ST_ERR_ECC_CORRECTED_VC15` reader - NA"]
pub type StErrEccCorrectedVc15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc0(&self) -> StErrEccCorrectedVc0R {
        StErrEccCorrectedVc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc1(&self) -> StErrEccCorrectedVc1R {
        StErrEccCorrectedVc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc2(&self) -> StErrEccCorrectedVc2R {
        StErrEccCorrectedVc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc3(&self) -> StErrEccCorrectedVc3R {
        StErrEccCorrectedVc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc4(&self) -> StErrEccCorrectedVc4R {
        StErrEccCorrectedVc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc5(&self) -> StErrEccCorrectedVc5R {
        StErrEccCorrectedVc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc6(&self) -> StErrEccCorrectedVc6R {
        StErrEccCorrectedVc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc7(&self) -> StErrEccCorrectedVc7R {
        StErrEccCorrectedVc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc8(&self) -> StErrEccCorrectedVc8R {
        StErrEccCorrectedVc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc9(&self) -> StErrEccCorrectedVc9R {
        StErrEccCorrectedVc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc10(&self) -> StErrEccCorrectedVc10R {
        StErrEccCorrectedVc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc11(&self) -> StErrEccCorrectedVc11R {
        StErrEccCorrectedVc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc12(&self) -> StErrEccCorrectedVc12R {
        StErrEccCorrectedVc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc13(&self) -> StErrEccCorrectedVc13R {
        StErrEccCorrectedVc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc14(&self) -> StErrEccCorrectedVc14R {
        StErrEccCorrectedVc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc15(&self) -> StErrEccCorrectedVc15R {
        StErrEccCorrectedVc15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_ecc_corrected::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStEccCorrectedSpec;
impl crate::RegisterSpec for IntStEccCorrectedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_ecc_corrected::R`](R) reader structure"]
impl crate::Readable for IntStEccCorrectedSpec {}
#[doc = "`reset()` method sets INT_ST_ECC_CORRECTED to value 0"]
impl crate::Resettable for IntStEccCorrectedSpec {}

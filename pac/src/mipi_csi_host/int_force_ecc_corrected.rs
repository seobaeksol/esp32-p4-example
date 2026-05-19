#[doc = "Register `INT_FORCE_ECC_CORRECTED` reader"]
pub type R = crate::R<IntForceEccCorrectedSpec>;
#[doc = "Register `INT_FORCE_ECC_CORRECTED` writer"]
pub type W = crate::W<IntForceEccCorrectedSpec>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC0` reader - NA"]
pub type ForceErrEccCorrectedVc0R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC0` writer - NA"]
pub type ForceErrEccCorrectedVc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC1` reader - NA"]
pub type ForceErrEccCorrectedVc1R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC1` writer - NA"]
pub type ForceErrEccCorrectedVc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC2` reader - NA"]
pub type ForceErrEccCorrectedVc2R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC2` writer - NA"]
pub type ForceErrEccCorrectedVc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC3` reader - NA"]
pub type ForceErrEccCorrectedVc3R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC3` writer - NA"]
pub type ForceErrEccCorrectedVc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC4` reader - NA"]
pub type ForceErrEccCorrectedVc4R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC4` writer - NA"]
pub type ForceErrEccCorrectedVc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC5` reader - NA"]
pub type ForceErrEccCorrectedVc5R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC5` writer - NA"]
pub type ForceErrEccCorrectedVc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC6` reader - NA"]
pub type ForceErrEccCorrectedVc6R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC6` writer - NA"]
pub type ForceErrEccCorrectedVc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC7` reader - NA"]
pub type ForceErrEccCorrectedVc7R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC7` writer - NA"]
pub type ForceErrEccCorrectedVc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC8` reader - NA"]
pub type ForceErrEccCorrectedVc8R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC8` writer - NA"]
pub type ForceErrEccCorrectedVc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC9` reader - NA"]
pub type ForceErrEccCorrectedVc9R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC9` writer - NA"]
pub type ForceErrEccCorrectedVc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC10` reader - NA"]
pub type ForceErrEccCorrectedVc10R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC10` writer - NA"]
pub type ForceErrEccCorrectedVc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC11` reader - NA"]
pub type ForceErrEccCorrectedVc11R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC11` writer - NA"]
pub type ForceErrEccCorrectedVc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC12` reader - NA"]
pub type ForceErrEccCorrectedVc12R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC12` writer - NA"]
pub type ForceErrEccCorrectedVc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC13` reader - NA"]
pub type ForceErrEccCorrectedVc13R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC13` writer - NA"]
pub type ForceErrEccCorrectedVc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC14` reader - NA"]
pub type ForceErrEccCorrectedVc14R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC14` writer - NA"]
pub type ForceErrEccCorrectedVc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC15` reader - NA"]
pub type ForceErrEccCorrectedVc15R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_CORRECTED_VC15` writer - NA"]
pub type ForceErrEccCorrectedVc15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc0(&self) -> ForceErrEccCorrectedVc0R {
        ForceErrEccCorrectedVc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc1(&self) -> ForceErrEccCorrectedVc1R {
        ForceErrEccCorrectedVc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc2(&self) -> ForceErrEccCorrectedVc2R {
        ForceErrEccCorrectedVc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc3(&self) -> ForceErrEccCorrectedVc3R {
        ForceErrEccCorrectedVc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc4(&self) -> ForceErrEccCorrectedVc4R {
        ForceErrEccCorrectedVc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc5(&self) -> ForceErrEccCorrectedVc5R {
        ForceErrEccCorrectedVc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc6(&self) -> ForceErrEccCorrectedVc6R {
        ForceErrEccCorrectedVc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc7(&self) -> ForceErrEccCorrectedVc7R {
        ForceErrEccCorrectedVc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc8(&self) -> ForceErrEccCorrectedVc8R {
        ForceErrEccCorrectedVc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc9(&self) -> ForceErrEccCorrectedVc9R {
        ForceErrEccCorrectedVc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc10(&self) -> ForceErrEccCorrectedVc10R {
        ForceErrEccCorrectedVc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc11(&self) -> ForceErrEccCorrectedVc11R {
        ForceErrEccCorrectedVc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc12(&self) -> ForceErrEccCorrectedVc12R {
        ForceErrEccCorrectedVc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc13(&self) -> ForceErrEccCorrectedVc13R {
        ForceErrEccCorrectedVc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc14(&self) -> ForceErrEccCorrectedVc14R {
        ForceErrEccCorrectedVc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc15(&self) -> ForceErrEccCorrectedVc15R {
        ForceErrEccCorrectedVc15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc0(
        &mut self,
    ) -> ForceErrEccCorrectedVc0W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc0W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc1(
        &mut self,
    ) -> ForceErrEccCorrectedVc1W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc1W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc2(
        &mut self,
    ) -> ForceErrEccCorrectedVc2W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc2W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc3(
        &mut self,
    ) -> ForceErrEccCorrectedVc3W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc3W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc4(
        &mut self,
    ) -> ForceErrEccCorrectedVc4W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc4W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc5(
        &mut self,
    ) -> ForceErrEccCorrectedVc5W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc5W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc6(
        &mut self,
    ) -> ForceErrEccCorrectedVc6W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc6W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc7(
        &mut self,
    ) -> ForceErrEccCorrectedVc7W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc7W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc8(
        &mut self,
    ) -> ForceErrEccCorrectedVc8W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc8W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc9(
        &mut self,
    ) -> ForceErrEccCorrectedVc9W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc9W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc10(
        &mut self,
    ) -> ForceErrEccCorrectedVc10W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc10W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc11(
        &mut self,
    ) -> ForceErrEccCorrectedVc11W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc11W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc12(
        &mut self,
    ) -> ForceErrEccCorrectedVc12W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc12W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc13(
        &mut self,
    ) -> ForceErrEccCorrectedVc13W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc13W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc14(
        &mut self,
    ) -> ForceErrEccCorrectedVc14W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc14W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc15(
        &mut self,
    ) -> ForceErrEccCorrectedVc15W<'_, IntForceEccCorrectedSpec> {
        ForceErrEccCorrectedVc15W::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_ecc_corrected::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_ecc_corrected::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForceEccCorrectedSpec;
impl crate::RegisterSpec for IntForceEccCorrectedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_ecc_corrected::R`](R) reader structure"]
impl crate::Readable for IntForceEccCorrectedSpec {}
#[doc = "`write(|w| ..)` method takes [`int_force_ecc_corrected::W`](W) writer structure"]
impl crate::Writable for IntForceEccCorrectedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_FORCE_ECC_CORRECTED to value 0"]
impl crate::Resettable for IntForceEccCorrectedSpec {}

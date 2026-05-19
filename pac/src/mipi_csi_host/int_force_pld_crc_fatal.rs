#[doc = "Register `INT_FORCE_PLD_CRC_FATAL` reader"]
pub type R = crate::R<IntForcePldCrcFatalSpec>;
#[doc = "Register `INT_FORCE_PLD_CRC_FATAL` writer"]
pub type W = crate::W<IntForcePldCrcFatalSpec>;
#[doc = "Field `FORCE_ERR_CRC_VC0` reader - NA"]
pub type ForceErrCrcVc0R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC0` writer - NA"]
pub type ForceErrCrcVc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC1` reader - NA"]
pub type ForceErrCrcVc1R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC1` writer - NA"]
pub type ForceErrCrcVc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC2` reader - NA"]
pub type ForceErrCrcVc2R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC2` writer - NA"]
pub type ForceErrCrcVc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC3` reader - NA"]
pub type ForceErrCrcVc3R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC3` writer - NA"]
pub type ForceErrCrcVc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC4` reader - NA"]
pub type ForceErrCrcVc4R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC4` writer - NA"]
pub type ForceErrCrcVc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC5` reader - NA"]
pub type ForceErrCrcVc5R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC5` writer - NA"]
pub type ForceErrCrcVc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC6` reader - NA"]
pub type ForceErrCrcVc6R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC6` writer - NA"]
pub type ForceErrCrcVc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC7` reader - NA"]
pub type ForceErrCrcVc7R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC7` writer - NA"]
pub type ForceErrCrcVc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC8` reader - NA"]
pub type ForceErrCrcVc8R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC8` writer - NA"]
pub type ForceErrCrcVc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC9` reader - NA"]
pub type ForceErrCrcVc9R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC9` writer - NA"]
pub type ForceErrCrcVc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC10` reader - NA"]
pub type ForceErrCrcVc10R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC10` writer - NA"]
pub type ForceErrCrcVc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC11` reader - NA"]
pub type ForceErrCrcVc11R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC11` writer - NA"]
pub type ForceErrCrcVc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC12` reader - NA"]
pub type ForceErrCrcVc12R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC12` writer - NA"]
pub type ForceErrCrcVc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC13` reader - NA"]
pub type ForceErrCrcVc13R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC13` writer - NA"]
pub type ForceErrCrcVc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC14` reader - NA"]
pub type ForceErrCrcVc14R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC14` writer - NA"]
pub type ForceErrCrcVc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC15` reader - NA"]
pub type ForceErrCrcVc15R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC15` writer - NA"]
pub type ForceErrCrcVc15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc0(&self) -> ForceErrCrcVc0R {
        ForceErrCrcVc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc1(&self) -> ForceErrCrcVc1R {
        ForceErrCrcVc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc2(&self) -> ForceErrCrcVc2R {
        ForceErrCrcVc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc3(&self) -> ForceErrCrcVc3R {
        ForceErrCrcVc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc4(&self) -> ForceErrCrcVc4R {
        ForceErrCrcVc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc5(&self) -> ForceErrCrcVc5R {
        ForceErrCrcVc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc6(&self) -> ForceErrCrcVc6R {
        ForceErrCrcVc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc7(&self) -> ForceErrCrcVc7R {
        ForceErrCrcVc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc8(&self) -> ForceErrCrcVc8R {
        ForceErrCrcVc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc9(&self) -> ForceErrCrcVc9R {
        ForceErrCrcVc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc10(&self) -> ForceErrCrcVc10R {
        ForceErrCrcVc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc11(&self) -> ForceErrCrcVc11R {
        ForceErrCrcVc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc12(&self) -> ForceErrCrcVc12R {
        ForceErrCrcVc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc13(&self) -> ForceErrCrcVc13R {
        ForceErrCrcVc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc14(&self) -> ForceErrCrcVc14R {
        ForceErrCrcVc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc15(&self) -> ForceErrCrcVc15R {
        ForceErrCrcVc15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc0(&mut self) -> ForceErrCrcVc0W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc0W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc1(&mut self) -> ForceErrCrcVc1W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc1W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc2(&mut self) -> ForceErrCrcVc2W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc2W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc3(&mut self) -> ForceErrCrcVc3W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc3W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc4(&mut self) -> ForceErrCrcVc4W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc4W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc5(&mut self) -> ForceErrCrcVc5W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc5W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc6(&mut self) -> ForceErrCrcVc6W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc6W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc7(&mut self) -> ForceErrCrcVc7W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc7W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc8(&mut self) -> ForceErrCrcVc8W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc8W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc9(&mut self) -> ForceErrCrcVc9W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc9W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc10(&mut self) -> ForceErrCrcVc10W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc10W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc11(&mut self) -> ForceErrCrcVc11W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc11W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc12(&mut self) -> ForceErrCrcVc12W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc12W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc13(&mut self) -> ForceErrCrcVc13W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc13W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc14(&mut self) -> ForceErrCrcVc14W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc14W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc15(&mut self) -> ForceErrCrcVc15W<'_, IntForcePldCrcFatalSpec> {
        ForceErrCrcVc15W::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_pld_crc_fatal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_pld_crc_fatal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForcePldCrcFatalSpec;
impl crate::RegisterSpec for IntForcePldCrcFatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_pld_crc_fatal::R`](R) reader structure"]
impl crate::Readable for IntForcePldCrcFatalSpec {}
#[doc = "`write(|w| ..)` method takes [`int_force_pld_crc_fatal::W`](W) writer structure"]
impl crate::Writable for IntForcePldCrcFatalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_FORCE_PLD_CRC_FATAL to value 0"]
impl crate::Resettable for IntForcePldCrcFatalSpec {}

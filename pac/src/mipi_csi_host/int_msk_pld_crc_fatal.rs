#[doc = "Register `INT_MSK_PLD_CRC_FATAL` reader"]
pub type R = crate::R<IntMskPldCrcFatalSpec>;
#[doc = "Register `INT_MSK_PLD_CRC_FATAL` writer"]
pub type W = crate::W<IntMskPldCrcFatalSpec>;
#[doc = "Field `MASK_ERR_CRC_VC0` reader - NA"]
pub type MaskErrCrcVc0R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC0` writer - NA"]
pub type MaskErrCrcVc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC1` reader - NA"]
pub type MaskErrCrcVc1R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC1` writer - NA"]
pub type MaskErrCrcVc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC2` reader - NA"]
pub type MaskErrCrcVc2R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC2` writer - NA"]
pub type MaskErrCrcVc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC3` reader - NA"]
pub type MaskErrCrcVc3R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC3` writer - NA"]
pub type MaskErrCrcVc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC4` reader - NA"]
pub type MaskErrCrcVc4R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC4` writer - NA"]
pub type MaskErrCrcVc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC5` reader - NA"]
pub type MaskErrCrcVc5R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC5` writer - NA"]
pub type MaskErrCrcVc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC6` reader - NA"]
pub type MaskErrCrcVc6R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC6` writer - NA"]
pub type MaskErrCrcVc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC7` reader - NA"]
pub type MaskErrCrcVc7R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC7` writer - NA"]
pub type MaskErrCrcVc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC8` reader - NA"]
pub type MaskErrCrcVc8R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC8` writer - NA"]
pub type MaskErrCrcVc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC9` reader - NA"]
pub type MaskErrCrcVc9R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC9` writer - NA"]
pub type MaskErrCrcVc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC10` reader - NA"]
pub type MaskErrCrcVc10R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC10` writer - NA"]
pub type MaskErrCrcVc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC11` reader - NA"]
pub type MaskErrCrcVc11R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC11` writer - NA"]
pub type MaskErrCrcVc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC12` reader - NA"]
pub type MaskErrCrcVc12R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC12` writer - NA"]
pub type MaskErrCrcVc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC13` reader - NA"]
pub type MaskErrCrcVc13R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC13` writer - NA"]
pub type MaskErrCrcVc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC14` reader - NA"]
pub type MaskErrCrcVc14R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC14` writer - NA"]
pub type MaskErrCrcVc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ERR_CRC_VC15` reader - NA"]
pub type MaskErrCrcVc15R = crate::BitReader;
#[doc = "Field `MASK_ERR_CRC_VC15` writer - NA"]
pub type MaskErrCrcVc15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc0(&self) -> MaskErrCrcVc0R {
        MaskErrCrcVc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc1(&self) -> MaskErrCrcVc1R {
        MaskErrCrcVc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc2(&self) -> MaskErrCrcVc2R {
        MaskErrCrcVc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc3(&self) -> MaskErrCrcVc3R {
        MaskErrCrcVc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc4(&self) -> MaskErrCrcVc4R {
        MaskErrCrcVc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc5(&self) -> MaskErrCrcVc5R {
        MaskErrCrcVc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc6(&self) -> MaskErrCrcVc6R {
        MaskErrCrcVc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc7(&self) -> MaskErrCrcVc7R {
        MaskErrCrcVc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc8(&self) -> MaskErrCrcVc8R {
        MaskErrCrcVc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc9(&self) -> MaskErrCrcVc9R {
        MaskErrCrcVc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc10(&self) -> MaskErrCrcVc10R {
        MaskErrCrcVc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc11(&self) -> MaskErrCrcVc11R {
        MaskErrCrcVc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc12(&self) -> MaskErrCrcVc12R {
        MaskErrCrcVc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc13(&self) -> MaskErrCrcVc13R {
        MaskErrCrcVc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc14(&self) -> MaskErrCrcVc14R {
        MaskErrCrcVc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc15(&self) -> MaskErrCrcVc15R {
        MaskErrCrcVc15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc0(&mut self) -> MaskErrCrcVc0W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc0W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc1(&mut self) -> MaskErrCrcVc1W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc1W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc2(&mut self) -> MaskErrCrcVc2W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc2W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc3(&mut self) -> MaskErrCrcVc3W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc3W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc4(&mut self) -> MaskErrCrcVc4W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc4W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc5(&mut self) -> MaskErrCrcVc5W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc5W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc6(&mut self) -> MaskErrCrcVc6W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc6W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc7(&mut self) -> MaskErrCrcVc7W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc7W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc8(&mut self) -> MaskErrCrcVc8W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc8W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc9(&mut self) -> MaskErrCrcVc9W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc9W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc10(&mut self) -> MaskErrCrcVc10W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc10W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc11(&mut self) -> MaskErrCrcVc11W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc11W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc12(&mut self) -> MaskErrCrcVc12W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc12W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc13(&mut self) -> MaskErrCrcVc13W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc13W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc14(&mut self) -> MaskErrCrcVc14W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc14W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn mask_err_crc_vc15(&mut self) -> MaskErrCrcVc15W<'_, IntMskPldCrcFatalSpec> {
        MaskErrCrcVc15W::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_pld_crc_fatal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_pld_crc_fatal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMskPldCrcFatalSpec;
impl crate::RegisterSpec for IntMskPldCrcFatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_msk_pld_crc_fatal::R`](R) reader structure"]
impl crate::Readable for IntMskPldCrcFatalSpec {}
#[doc = "`write(|w| ..)` method takes [`int_msk_pld_crc_fatal::W`](W) writer structure"]
impl crate::Writable for IntMskPldCrcFatalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_MSK_PLD_CRC_FATAL to value 0"]
impl crate::Resettable for IntMskPldCrcFatalSpec {}

#[doc = "Register `INT_FORCE_SEQ_FRAME_FATAL` reader"]
pub type R = crate::R<IntForceSeqFrameFatalSpec>;
#[doc = "Register `INT_FORCE_SEQ_FRAME_FATAL` writer"]
pub type W = crate::W<IntForceSeqFrameFatalSpec>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC0` reader - NA"]
pub type ForceErrFSeqVc0R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC0` writer - NA"]
pub type ForceErrFSeqVc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC1` reader - NA"]
pub type ForceErrFSeqVc1R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC1` writer - NA"]
pub type ForceErrFSeqVc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC2` reader - NA"]
pub type ForceErrFSeqVc2R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC2` writer - NA"]
pub type ForceErrFSeqVc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC3` reader - NA"]
pub type ForceErrFSeqVc3R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC3` writer - NA"]
pub type ForceErrFSeqVc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC4` reader - NA"]
pub type ForceErrFSeqVc4R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC4` writer - NA"]
pub type ForceErrFSeqVc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC5` reader - NA"]
pub type ForceErrFSeqVc5R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC5` writer - NA"]
pub type ForceErrFSeqVc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC6` reader - NA"]
pub type ForceErrFSeqVc6R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC6` writer - NA"]
pub type ForceErrFSeqVc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC7` reader - NA"]
pub type ForceErrFSeqVc7R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC7` writer - NA"]
pub type ForceErrFSeqVc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC8` reader - NA"]
pub type ForceErrFSeqVc8R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC8` writer - NA"]
pub type ForceErrFSeqVc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC9` reader - NA"]
pub type ForceErrFSeqVc9R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC9` writer - NA"]
pub type ForceErrFSeqVc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC10` reader - NA"]
pub type ForceErrFSeqVc10R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC10` writer - NA"]
pub type ForceErrFSeqVc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC11` reader - NA"]
pub type ForceErrFSeqVc11R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC11` writer - NA"]
pub type ForceErrFSeqVc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC12` reader - NA"]
pub type ForceErrFSeqVc12R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC12` writer - NA"]
pub type ForceErrFSeqVc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC13` reader - NA"]
pub type ForceErrFSeqVc13R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC13` writer - NA"]
pub type ForceErrFSeqVc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC14` reader - NA"]
pub type ForceErrFSeqVc14R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC14` writer - NA"]
pub type ForceErrFSeqVc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_F_SEQ_VC15` reader - NA"]
pub type ForceErrFSeqVc15R = crate::BitReader;
#[doc = "Field `FORCE_ERR_F_SEQ_VC15` writer - NA"]
pub type ForceErrFSeqVc15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc0(&self) -> ForceErrFSeqVc0R {
        ForceErrFSeqVc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc1(&self) -> ForceErrFSeqVc1R {
        ForceErrFSeqVc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc2(&self) -> ForceErrFSeqVc2R {
        ForceErrFSeqVc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc3(&self) -> ForceErrFSeqVc3R {
        ForceErrFSeqVc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc4(&self) -> ForceErrFSeqVc4R {
        ForceErrFSeqVc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc5(&self) -> ForceErrFSeqVc5R {
        ForceErrFSeqVc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc6(&self) -> ForceErrFSeqVc6R {
        ForceErrFSeqVc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc7(&self) -> ForceErrFSeqVc7R {
        ForceErrFSeqVc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc8(&self) -> ForceErrFSeqVc8R {
        ForceErrFSeqVc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc9(&self) -> ForceErrFSeqVc9R {
        ForceErrFSeqVc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc10(&self) -> ForceErrFSeqVc10R {
        ForceErrFSeqVc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc11(&self) -> ForceErrFSeqVc11R {
        ForceErrFSeqVc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc12(&self) -> ForceErrFSeqVc12R {
        ForceErrFSeqVc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc13(&self) -> ForceErrFSeqVc13R {
        ForceErrFSeqVc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc14(&self) -> ForceErrFSeqVc14R {
        ForceErrFSeqVc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc15(&self) -> ForceErrFSeqVc15R {
        ForceErrFSeqVc15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc0(&mut self) -> ForceErrFSeqVc0W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc0W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc1(&mut self) -> ForceErrFSeqVc1W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc1W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc2(&mut self) -> ForceErrFSeqVc2W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc2W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc3(&mut self) -> ForceErrFSeqVc3W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc3W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc4(&mut self) -> ForceErrFSeqVc4W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc4W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc5(&mut self) -> ForceErrFSeqVc5W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc5W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc6(&mut self) -> ForceErrFSeqVc6W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc6W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc7(&mut self) -> ForceErrFSeqVc7W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc7W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc8(&mut self) -> ForceErrFSeqVc8W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc8W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc9(&mut self) -> ForceErrFSeqVc9W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc9W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc10(&mut self) -> ForceErrFSeqVc10W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc10W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc11(&mut self) -> ForceErrFSeqVc11W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc11W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc12(&mut self) -> ForceErrFSeqVc12W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc12W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc13(&mut self) -> ForceErrFSeqVc13W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc13W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc14(&mut self) -> ForceErrFSeqVc14W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc14W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn force_err_f_seq_vc15(&mut self) -> ForceErrFSeqVc15W<'_, IntForceSeqFrameFatalSpec> {
        ForceErrFSeqVc15W::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_seq_frame_fatal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_seq_frame_fatal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForceSeqFrameFatalSpec;
impl crate::RegisterSpec for IntForceSeqFrameFatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_seq_frame_fatal::R`](R) reader structure"]
impl crate::Readable for IntForceSeqFrameFatalSpec {}
#[doc = "`write(|w| ..)` method takes [`int_force_seq_frame_fatal::W`](W) writer structure"]
impl crate::Writable for IntForceSeqFrameFatalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_FORCE_SEQ_FRAME_FATAL to value 0"]
impl crate::Resettable for IntForceSeqFrameFatalSpec {}

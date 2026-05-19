#[doc = "Register `PHY_TST_CTRL1` reader"]
pub type R = crate::R<PhyTstCtrl1Spec>;
#[doc = "Register `PHY_TST_CTRL1` writer"]
pub type W = crate::W<PhyTstCtrl1Spec>;
#[doc = "Field `PHY_TESTDIN` reader - NA"]
pub type PhyTestdinR = crate::FieldReader;
#[doc = "Field `PHY_TESTDIN` writer - NA"]
pub type PhyTestdinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHT_TESTDOUT` reader - NA"]
pub type PhtTestdoutR = crate::FieldReader;
#[doc = "Field `PHY_TESTEN` reader - NA"]
pub type PhyTestenR = crate::BitReader;
#[doc = "Field `PHY_TESTEN` writer - NA"]
pub type PhyTestenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn phy_testdin(&self) -> PhyTestdinR {
        PhyTestdinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn pht_testdout(&self) -> PhtTestdoutR {
        PhtTestdoutR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn phy_testen(&self) -> PhyTestenR {
        PhyTestenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn phy_testdin(&mut self) -> PhyTestdinW<'_, PhyTstCtrl1Spec> {
        PhyTestdinW::new(self, 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn phy_testen(&mut self) -> PhyTestenW<'_, PhyTstCtrl1Spec> {
        PhyTestenW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_tst_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_tst_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTstCtrl1Spec;
impl crate::RegisterSpec for PhyTstCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tst_ctrl1::R`](R) reader structure"]
impl crate::Readable for PhyTstCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_tst_ctrl1::W`](W) writer structure"]
impl crate::Writable for PhyTstCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_TST_CTRL1 to value 0"]
impl crate::Resettable for PhyTstCtrl1Spec {}

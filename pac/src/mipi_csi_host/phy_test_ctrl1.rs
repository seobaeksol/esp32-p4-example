#[doc = "Register `PHY_TEST_CTRL1` reader"]
pub type R = crate::R<PhyTestCtrl1Spec>;
#[doc = "Register `PHY_TEST_CTRL1` writer"]
pub type W = crate::W<PhyTestCtrl1Spec>;
#[doc = "Field `PHY_TESTDIN` reader - NA"]
pub type PhyTestdinR = crate::FieldReader;
#[doc = "Field `PHY_TESTDIN` writer - NA"]
pub type PhyTestdinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_TESTDOUT` reader - NA"]
pub type PhyTestdoutR = crate::FieldReader;
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
    pub fn phy_testdout(&self) -> PhyTestdoutR {
        PhyTestdoutR::new(((self.bits >> 8) & 0xff) as u8)
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
    pub fn phy_testdin(&mut self) -> PhyTestdinW<'_, PhyTestCtrl1Spec> {
        PhyTestdinW::new(self, 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn phy_testen(&mut self) -> PhyTestenW<'_, PhyTestCtrl1Spec> {
        PhyTestenW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_test_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_test_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTestCtrl1Spec;
impl crate::RegisterSpec for PhyTestCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_test_ctrl1::R`](R) reader structure"]
impl crate::Readable for PhyTestCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_test_ctrl1::W`](W) writer structure"]
impl crate::Writable for PhyTestCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_TEST_CTRL1 to value 0"]
impl crate::Resettable for PhyTestCtrl1Spec {}

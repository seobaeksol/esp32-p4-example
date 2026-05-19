#[doc = "Register `PHY_TEST_CTRL0` reader"]
pub type R = crate::R<PhyTestCtrl0Spec>;
#[doc = "Register `PHY_TEST_CTRL0` writer"]
pub type W = crate::W<PhyTestCtrl0Spec>;
#[doc = "Field `PHY_TESTCLR` reader - NA"]
pub type PhyTestclrR = crate::BitReader;
#[doc = "Field `PHY_TESTCLR` writer - NA"]
pub type PhyTestclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TESTCLK` reader - NA"]
pub type PhyTestclkR = crate::BitReader;
#[doc = "Field `PHY_TESTCLK` writer - NA"]
pub type PhyTestclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_testclr(&self) -> PhyTestclrR {
        PhyTestclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_testclk(&self) -> PhyTestclkR {
        PhyTestclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_testclr(&mut self) -> PhyTestclrW<'_, PhyTestCtrl0Spec> {
        PhyTestclrW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_testclk(&mut self) -> PhyTestclkW<'_, PhyTestCtrl0Spec> {
        PhyTestclkW::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_test_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_test_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTestCtrl0Spec;
impl crate::RegisterSpec for PhyTestCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_test_ctrl0::R`](R) reader structure"]
impl crate::Readable for PhyTestCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_test_ctrl0::W`](W) writer structure"]
impl crate::Writable for PhyTestCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_TEST_CTRL0 to value 0x01"]
impl crate::Resettable for PhyTestCtrl0Spec {
    const RESET_VALUE: u32 = 0x01;
}
